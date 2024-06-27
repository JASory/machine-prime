#[cfg(not(any(feature = "small", feature = "tiny")))]
use crate::hashbase::FERMAT_BASE;

#[cfg(feature ="ssmr")]
use crate::hashbase::FERMAT_BASE_40;

#[cfg(not(feature = "tiny"))]
use crate::primes::{INV_8,PRIME_INV_64};


 fn mod_inv(n: u64) -> u64 {
    #[cfg(feature = "tiny")]
    {
        let mut est = (3 * n) ^ 2;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est.wrapping_neg()
    }

    #[cfg(not(feature = "tiny"))]
    {
        let mut est = INV_8[((n >> 1) & 0x7F) as usize] as u64;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est.wrapping_neg()
    }
}


 fn mont_prod(x: u64, y: u64, n: u64, npi: u64) -> u64 {
    let interim = x as u128 * y as u128;
    let tm = (interim as u64).wrapping_mul(npi);
    let (t, flag) = interim.overflowing_add((tm as u128) * (n as u128));
    let t = (t >> 64) as u64;
    
    if flag {
        t + n.wrapping_neg()
    } else if t >= n {
        t - n
    } else {
        t
    }
}

fn to_normal_form(x: u64, n: u64, npi: u64) -> u64{
    let tm = x.wrapping_mul(npi);
    let (t, flag) = (x as u128).overflowing_add((tm as u128) * (n as u128));
    let t = (t >> 64) as u64;
    
    if flag {
        t + n.wrapping_neg()
    } else if t >= n {
        t - n
    } else {
        t
    }
}


fn mont_sub(x: u64, y: u64, n: u64) -> u64 {
    if y > x {
        return n.wrapping_sub(y.wrapping_sub(x));
    }
    x.wrapping_sub(y)
}

#[cfg(any(feature = "small", feature = "tiny"))]
fn jacobi(a: u64, k: u64) -> i8 {
    let mut n = a;
    let mut p = k;
    let mut t = 1i8;
    n %= p;

    while n != 0 {
        let zeros = n.trailing_zeros();
        n >>= zeros;

        if (p % 8 == 3 || p % 8 == 5) && (zeros % 2 == 1) {
            t = -t
        }
        
        let interim = p;
        p = n;
        n = interim;

        if n % 4 == 3 && p % 4 == 3 {
            t = -t;
        }
        n %= p;
    }

    if p == 1 {
        t
    } else {
        0
    }
}

#[cfg(any(feature = "small", feature = "tiny"))]
fn param_search(n: u64) -> u64 {
    let mut d: u64;
    let mut p = 3;

    loop {
        d = p * p - 4;
        let sym = jacobi(d, n);
        if sym == -1 {
            break;
        }
        p += 1;
    } 
    p
}

// Only Small and Tiny modes use Lucas sequence test
// Perfect squares that fail 1194649, 12327121,
#[cfg(any(feature = "small", feature = "tiny"))]
 fn lucas(n: u64,npi: u64) -> bool {
    let param = param_search(n);
    let s = (n + 1).trailing_zeros();
    let d = (n + 1) >> s;

    // Montgomery forms of starting parameter, 2, and n-2
    let m_param = (((param as u128) << 64) % (n as u128)) as u64;
    let m_2 = ((2u128 << 64) % (n as u128)) as u64;
    let m_2_inv = ((((n - 2) as u128) << 64) % (n as u128)) as u64;
    
    let mut w = mont_sub(mont_prod(m_param, m_param, n, npi), m_2, n);
    let mut v = m_param;
    let b = 64 - d.leading_zeros();

    for i in 2..(b + 1) {
        let t = mont_sub(mont_prod(v, w, n, npi), m_param, n);

        if (d >> (b - i)) & 1 == 1 {
            v = t;
            w = mont_sub(mont_prod(w, w, n, npi), m_2, n);
        } else {
            w = t;
            v = mont_sub(mont_prod(v, v, n, npi), m_2, n);
        }
    }

    if v == m_2 || v == m_2_inv {
        return true;
    }

    for _ in 1..s {
        if v == 0 {
            return true;
        }
        v = mont_sub(mont_prod(v, v, n, npi), m_2, n);
        if v == m_2 {
            return false;
        }
    }
    false
}



//  In: 
// Out:

fn mont_pow(mut base: u64, mut one: u64, mut p: u64, n: u64, npi: u64) -> u64 {
    
  while p > 1 {
        if p & 1 == 0 {
            base = mont_prod(base, base, n, npi);
            p >>= 1;
        } else {
            one = mont_prod(one, base, n, npi);
            base = mont_prod(base, base, n, npi);
            p = (p - 1) >> 1;
        }
    }
    mont_prod(one, base, n, npi)
} 

// All modes call this function
fn euler_p(p: u64, one: u64, npi: u64) -> bool {
//    panic!("NOP E_P");
    let res = p & 7;
    let mut param = 0;

    if res == 1 {
        param = 1;
    }
      // Montgomery form of 2
    let mut result = one.wrapping_add(one);
   
    let d = (p - 1) >> (1 + param);

    result = mont_pow(result,one,d,p,npi);

    result = to_normal_form(result,p,npi);//mont_prod(result, 1, p, npi);
 
    if result == 1 {
        return res == 1 || res == 7;
    } else if result == p - 1 {
        return res == 1 || res == 3 || res == 5;
    }
    false
}



// Only the Default mode calls this function
#[cfg(not(any(feature = "small", feature = "tiny")))]
fn sprp(p: u64, base: u64, one: u64, npi: u64) -> bool {
    let p_minus = p - 1;
    let twofactor = p_minus.trailing_zeros();
    let d = p_minus >> twofactor;

    let mut result = base.wrapping_mul(one);
    
    let oneinv = mont_prod(mont_sub(p,one,p),one,p,npi);
    
    result = mont_pow(result,one,d,p,npi);
    
    
    if result == one || result == oneinv {
        return true;
    }

    for _ in 1..twofactor {
        result = mont_prod(result, result, p, npi);

        if result == oneinv {
            return true;
        }
    }
    false
}

fn core_primality(x: u64) -> bool{

  let npi = mod_inv(x);
  let one = (u64::MAX % x) + 1;
 
 #[cfg(feature="ssmr")]
 {
   if x < 1099720565341{
    let idx = (x as u32).wrapping_mul(2202620065).wrapping_shr(19) as usize;
    return sprp(x, FERMAT_BASE_40[idx] as u64, one,npi)
   }
 }

  if !euler_p(x,one,npi){
     return false;
  }
  
  #[cfg(not(any(feature = "small", feature = "tiny")))]
    {
        let idx = (x as u32).wrapping_mul(1276800789).wrapping_shr(14) as usize;
         sprp(x, FERMAT_BASE[idx] as u64, one,npi)
    }

    #[cfg(any(feature = "small", feature = "tiny"))]
    {
        lucas(x,npi)
    }  
    
}


/// Primality testing optimized for the average case in the interval 0;2^64.
/// Approximately 5 times faster than is_prime_wc in the average case, but slightly slower in the worst case.
#[no_mangle]
pub extern "C" fn is_prime(x: u64) -> bool {
    if x == 1 {
        return false;
    }

    #[cfg(any(feature = "small", feature = "tiny"))]
    {
        // Two perfect squares that infinitely loop in Lucas test due to Jacobi symbol search
        // There is only 1 in 2^63 probability of being encountered at random
        if x == 1194649 || x == 12327121 {
            return false;
        }
    }

    if x == 2 {
        return true;
    }

    if x & 1 == 0 {
        return false;
    }

    #[cfg(not(feature = "tiny"))]
    {
     const TRIAL_BOUND : u64 = 55730344633563600;
     
     if x < TRIAL_BOUND {
     
            for i in PRIME_INV_64.iter() {
                let prod = x.wrapping_mul(*i);
                if prod == 1 {
                    return true;
                }
                if prod < x {
                    return false;
                }
            }
     }       
    
     if x >  TRIAL_BOUND {
     
      let interim = x%13082761331670030u64;
        
        for i in PRIME_INV_64[..13].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
        let interim = x%10575651537777253u64;
        
        for i in PRIME_INV_64[13..21].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
        let interim = x%9823972789433423u64;
        
        for i in PRIME_INV_64[21..29].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
        
        let interim = x%805474958639317u64;
        
        for i in PRIME_INV_64[29..35].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
       
        let interim = x%4575249731290429u64;
        
        for i in PRIME_INV_64[35..42].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
       
        let interim = x%18506541671175721u64;
        
        for i in PRIME_INV_64[42..49].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
        let interim = x%61247129307885343u64;
        
        for i in PRIME_INV_64[49..56].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
        
        let interim = x%536967265590991u64;
        
        for i in PRIME_INV_64[56..62].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }
        
        
        let interim = x%1194404299990379u64;
        
        for i in PRIME_INV_64[62..].iter(){
           if interim.wrapping_mul(*i) < interim{
             return false
           }
        }

        } //end if
        
    } // end conditional block

    core_primality(x)
}

/// Primality testing for the worst case. Panics at zero, flags 1 as prime, 2 as composite. Reduced memory variants 
/// infinitely loop if the input is one of the perfect squares 1194649 or 12327121.This option is intended for proving 
/// primality for integers that have already been checked using simpler methods.
/// For example one could generate random integers without small factors and then prove that they are prime faster than with
/// is_prime. Other applications include checking primality within a factorization function.
/// Approximately 13% faster against primes than is_prime. 
#[no_mangle]
pub extern "C" fn is_prime_wc(x: u64) -> bool {
    /*
    Alerts for the failure points
    compiled library from Makefile does not have this check
    */
     debug_assert!(x !=1 && x !=2 && x !=0);
     
     #[cfg(any(feature = "small", feature = "tiny"))]
     {
     debug_assert!(x != 1194649 && x != 12327121);
     }
  
      core_primality(x)
}


