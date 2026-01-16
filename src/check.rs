#[cfg(feature = "ssmr")]
use crate::hashbase::FERMAT_WITNESS;

#[cfg(any(feature = "lucas",feature = "ssmr"))]
use crate::primes::{INV_8,PRIME_TABLE};

#[cfg(all(feature = "lucas", not( feature = "ssmr")))]
use crate::primes::LUCAS_PARAM;

//#[cfg(feature = "wide")]
//use crate::double::{is_prime_128, is_prime_wc_128};

/// Multiplicative inverse over Z/2^64
///
///  In:  n \in 2Z + 1
///
/// Out: n^-1
pub const fn mul_inv2(n: u64) -> u64 {
    #[cfg(not(any(feature = "lucas", feature = "ssmr")))]
    {        assert!(false);
        let mut est: u64 = 3u64.wrapping_mul(n) ^ 2;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }

    #[cfg(any(feature = "lucas",feature = "ssmr"))]
    {           
    /*
       let mut est: u64 = 3*n^2;//INV_8[(n.wrapping_shr(1) & 0x7F) as usize] as u64;
       let mut y = 1u64.wrapping_sub(n.wrapping_mul(est));
       est*=1+y;
       y*=y;
       est*=1+y;
       y*=y;
       est*=1+y;
       y*=y;
       est*=1+y;
       est
       */
        let mut est: u64 = INV_8[(n.wrapping_shr(1) & 0x7F) as usize] as u64;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }
}

const fn widening_mul(x: u64, y: u64) -> (u64,u64){
      let prod = x as u128 * y as u128;
      (prod as u64, (prod >> 64) as u64)
}

/// Product in Montgomery form
///
/// In: Mont(X,N),Mont(Y,N), N^-1,N
///
/// Out: Mont(X*Y,N)
pub const fn mont_prod(x: u64, y: u64, inv: u64, n: u64) -> u64 {
    let (lo, hi) = widening_mul(x, y);
    // Then find a multiple of n with the same low-order word
    let (_, borrow) = widening_mul(lo.wrapping_mul(inv), n);
    mont_sub(hi, borrow, n)
}

/// Convert to Montgomery form
///
/// In: X, N
///
/// Out: Mont(X,N)
#[inline]
pub const fn to_mont(x: u64, n: u64) -> u64 {
   ( ( (x as u128) <<64) % (n as u128) ) as u64
}

/// One in Montgomery form
///
/// In: N
///
/// Out: Mont(1,N)
#[inline]
pub const fn one_mont(n: u64) -> u64 {
    n.wrapping_neg() % n
}

/// Two in Montgomery form
///
/// In: Mont(1,N), N
///
/// Out: Mont(2,N)
pub const fn two_mont(one: u64, n: u64) -> u64 {    
    let two = 2*one;    // Can't overflow due to way one is computed
    if two >= n {
        return two - n;
    }    
    two
}

/// Subtraction in Montgomery form   
///   
/// In: X,Y,N
///  
/// Out: X-Y mod N
pub const fn mont_sub(x: u64, y: u64, n: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        x.wrapping_sub(y).wrapping_add(n)
    }
}

/*
   Possible optimizations
   
   Since this is only used for searching for Selfridge's parameter, a is always of the form (p-2)(p+2)
   can we only evaluate one component and therefore restrict arithmetic to 32-bit?
   
   This algorithm is David Sparks' branchless variant of the original. 
*/

/// Check if non-quadratic residue
///
///  In: A,N where N is odd
///
/// Out: Jacobi(A,N) == -1
#[cfg(not(feature = "ssmr"))]
pub const fn nqr(mut a: u64, mut n: u64) -> bool {
 
     let mut sign : u32 = 0;
     
     while a != 0{
        // Remove factors of a that are powers of two 
        let twofactor = a.trailing_zeros();
        a>>=twofactor;
        // flip sign if p == 3 or 5 mod 8 and zeros is odd
        sign ^= (n as u32).wrapping_add(2) >> 1 & twofactor << 1;
        // if p == n == 3 mod 4 flip sign
        sign ^= (n & a) as u32;
        
        (a,n) = (n%a,a); 
     }
     n == 1 && sign & 2 != 0
 }

/// Lucas parameter search
///
///  In: N
///
/// Out: x := jacobi(x*x-4,N) == -1
#[cfg(not(feature = "ssmr"))]
pub const fn param_search(n: u64) -> u64 {
    #[cfg(feature = "lucas")]
    {
        let rem = n % 5;

        if rem == 3 || rem == 2 {
            return 3;
        }

        let rem = n % 12;

        if rem == 5 || rem == 7{
            return 4;
        }

        let rem = n % 21;

        if rem == 2 || rem == 8 || rem == 11 || rem == 10 || rem == 13 || rem == 19 {
            return 5;
        }
        // Search for p such that (p-2)(p+2) is  a nonquadratic residue to N
        // the Lucas_Param set accounts for all base-2 strong pseudoprimes less than 2^64
         
        let mut idx: usize = 0;
          
        while idx < 27 {
            let i = LUCAS_PARAM[idx] as u64;
            if nqr(i*i-4, n) {
                return i;
            }
            idx +=1;
        }
        0u64
    }

    #[cfg(not(feature = "lucas"))]
    {
        let mut d: u64;
        let mut p = 3u64;

        loop {
            d = p*p-4;
            if nqr(d, n) {
                break;
            }
            p +=1;
        }
        p
    }
}

///  Lucas-V sequence test with Selfridge parameters
/// 
/// In: N,Mont(1,N), Mont(2,N), N^-1
///
/// Out: Lucas_V(n)
#[cfg(not(feature = "ssmr"))]
pub const fn lucas(n: u64, one: u64, two: u64, inv: u64) -> bool {
    let param = param_search(n);

    #[cfg(feature = "lucas")]
    { // if a nonquadratic residue is not found in the LUCAS_PARAM table then n is prime 
        if param == 0 {
            return true;
        }
    }
    // 2^64 -1 is not a base-2 pseudoprime so this will not overflow
    let n_plus = n+1;
    let s = n_plus.trailing_zeros();
    let d = n_plus>>s;

    let m_param = to_mont(param, n);

    let m_2_inv = mont_prod(mont_sub(n, two, n), one, inv, n);

    let mut w = mont_sub(mont_prod(m_param, m_param, inv, n), two, n);
    let mut v = m_param;

    let b : u32 = 64-d.leading_zeros();

    let mut i = 2;

    while i < (b+1) {
        let t = mont_sub(mont_prod(v, w, inv, n), m_param, n);

        if (d>>(b-i)) & 1 == 1 {
            v = t;
            w = mont_sub(mont_prod(w, w, inv, n), two, n);
        } else {
            w = t;
            v = mont_sub(mont_prod(v, v, inv, n), two, n);
        }
        i +=1;
    }

    if v == two || v == m_2_inv {
        return true;
    }

    let mut counter = 1;

    while counter < s {
    
        if v == 0 {
            return true;
        }

        v = mont_sub(mont_prod(v, v, inv, n), two, n);
        
        if v == two {
            return false;
        }
        
        counter += 1;
    }
    false
}

/// Modular exponentiation in Montgomery form
///
///  In: Mont(base),Mont(1),pow, inv,n
///
/// Out: base^pow mod n
pub const fn mont_pow(mut base: u64, mut one: u64, mut pow: u64, inv: u64, n: u64) -> u64 {

    while pow > 1 {
        if pow & 1 == 0 {
            base = mont_prod(base, base, inv, n);
            pow >>=1;
        } else {
            one = mont_prod(one, base, inv, n);
            base = mont_prod(base, base, inv, n);
            pow >>= 1;
        }
    }
    mont_prod(one, base, inv, n)
}

/// Fermat witness selection for n < 2^64
#[cfg(feature = "ssmr")]
#[inline]
pub const fn witness_selector(x: u64) -> u64 {
    FERMAT_WITNESS[((x as u32).wrapping_mul(811484239)>>14) as usize] as u64
}

/// Strong Fermat test
///
/// In: N,tz := a*2^tz+1 =N, Mont(base,N), Mont(1,N), Mont(N-1,N),
///
/// Out: SPRP(N,base)
pub const fn strong_fermat(n: u64, tz: u32, base: u64, one: u64, oneinv: u64, inv: u64) -> bool {

    let d = n>>tz;

    let mut result = mont_pow(base, one, d, inv,n);

    if result == one || result == oneinv {
        return true;
    }

    let mut count = 1;

    while count < tz {
    
        count +=1;
        
        result = mont_prod(result, result, inv, n);

        if result == oneinv {
            return true;
        }
    }
    false
}

const fn core_primality(x: u64) -> bool {

    let inv = mul_inv2(x);

    let tzc = (x-1).trailing_zeros();
    
    let one = one_mont(x);
    
    let oneinv = x.wrapping_sub(one);
   
    #[cfg(feature = "ssmr")]
    {   // Due to sophisticated witness precomputation, Machine-prime can prove the integers 
        // less than 2^47 prime with only a single witness
      if x < 0x800000000000{
      
          let wit = witness_selector(x);
          
          return strong_fermat(x, tzc, to_mont(wit, x), one, oneinv, inv);  
      } else {
        // Witness 2 Strong Fermat test can be performed faster than a lookup witness
        let two = two_mont(one,x);
        
        if !strong_fermat(x,tzc,two,one,oneinv,inv){
           return false;
        }
        // Only primes and the very few 2-strong pseudoprimes are subjected to this test
        let wit = witness_selector(x);
        
        return strong_fermat(x, tzc, to_mont(wit, x), one, oneinv, inv);
      }
     }
    #[cfg(not(feature = "ssmr"))]
    {
        let two = two_mont(one, x);
        
        if !strong_fermat(x, tzc, two, one, oneinv, inv) {
            return false;
        }

        if x < 2047 {
            return true;
        }
        // check if x is a perfect square
        // This eliminates the case of x == 1194649 and x == 12327121
        // Which are squares of the Weiferich primes that pass a base-2 fermat test and infinitely loop
        // in the Selfridge search for the Lucas sequence test.
        let sqrt = x.isqrt();
        
        if x == sqrt*sqrt{
           return false;
        }

        lucas(x, one, two, inv)
    }
}

/// Primality testing optimized for the average case in the interval 0;2^64.
///
/// Approximately 5 times faster than is_prime_wc in the average case, but slightly slower in the worst case.
#[no_mangle]
pub const extern "C" fn is_prime(x: u64) -> bool {
    if x == 1 {
        return false;
    }

    if x == 2 {
        return true;
    }

    if x & 1 == 0 {
        return false;
    }

    #[cfg(any(feature = "lucas", feature = "ssmr"))]
    {
  
        let mut idx: usize = 0;
        
        while idx < 256 {
          // Multiply x by a prime inverse over 2^64
          let prod = x.wrapping_mul(PRIME_TABLE[idx]);
          // Check if prod <= 2^64/p, if so then it is either p itself resulting in prod == 1
          // or it is a composite divisible by p
          if prod <= PRIME_TABLE[idx+1]{
             return prod==1;
          }
          
           idx += 2;
        }

    } // end conditional block 

    core_primality(x)
}

/// Primality testing for the worst case.
///
/// Panics at zero, flags 1 as prime, 2 as composite.
/// # SSMR
/// May pass some even numbers as prime
/// # Lucas
/// No additional errors
/// # Tiny
/// No additional errors
#[no_mangle]
pub const extern "C" fn is_prime_wc(x: u64) -> bool {
    /*
    Alerts for the failure points
    compiled library from Makefile does not have this check
    */
    debug_assert!(x != 1 && x != 2 && x != 0);
    #[cfg(feature = "ssmr")]
    {
        debug_assert!(x&1==1);
    }

    core_primality(x)
}
