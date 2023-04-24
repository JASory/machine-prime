use crate::primes::{INV_8,PRIME_INV_64,PRIME_INV_128};
use crate::hashbase::FERMAT_BASE;


 fn mod_inv(n: u64) -> u64 {
    // inverse of odd n in  2^64
    let mut est = INV_8[((n >> 1) & 0x7F) as usize] as u64;
    est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
    est.wrapping_neg()
}

fn mont_prod_64(x: u64 , y: u64, n: u64, npi: u64) -> u64 {
   let interim = x as u128 * y as u128 ; 
   let tm = (interim as u64).wrapping_mul(npi);
   let (t, flag) = interim.overflowing_add((tm as u128) * (n as u128));
   let t = (t>>64) as u64; 
   if flag { 
    t + n.wrapping_neg() 
    } 
    else if t >= n {
     t-n 
    }    else {
    t
    } 
 }

 fn sprp(p: u64, base: u64) -> bool {
    let p_minus = p - 1;
    let twofactor = p_minus.trailing_zeros();
    let mut d = p_minus >> twofactor;

    let npi = mod_inv(p);
    let one = (u64::MAX%p)+1;
    let mut z = one; 
    let mut result = (((base as u128)<<64) % (p as u128)) as u64; 
    let oneinv = (((p_minus as u128)<<64) % (p as u128)) as u64; 
    
    while d > 1 {
      
      if d&1 == 0{
       result = mont_prod_64(result, result, p, npi);
       d >>=1;
      }
      else {
            z = mont_prod_64(z, result, p, npi);
       result = mont_prod_64(result, result, p, npi);
       d =(d-1)>>1;
      
      }
    }
    
     result = mont_prod_64(z,result,p,npi);
    
    if result == one ||  result == oneinv {
        return true;
    }
    
    for _ in 1..twofactor{
        result = mont_prod_64(result, result, p, npi);
       
        if result == oneinv {
            return true;
        }
    }
    false
   } 

/// Primality testing optimized for the average case in the interval 0;2^64. 
/// Approximately 5 times faster than is_prime_wc in the average case, but slightly slower in the worst case
#[no_mangle]
pub extern "C" fn is_prime_ac(x: u64) -> bool {
        if x == 1{
          return false
        }
        if x == 2{
          return true
        }
        if x & 1 == 0 {
            return false;
        }

        if x < 0x5A2553748E42E8 {
            for i in PRIME_INV_64.iter() {
             let prod = x.wrapping_mul(*i);
                if prod == 1{
                  return true
                }
                if prod < x {
                    return false;
                }
            }
        }

        if x > 0x5A2553748E42E8 {
            for i in PRIME_INV_128.iter() {
                if ((x as u128).wrapping_mul(*i)) < x as u128 {
                    return false;
                }
            }
        }

        if !sprp(x,2u64){
            return false;
        }

        let idx = ((x as u32).wrapping_mul(942441121)>>14) as usize;
        sprp(x,FERMAT_BASE[idx] as u64)
    }
/// Primality testing for the worst case. Panics at zero, flags 1 as prime, flags powers of 2 as prime.  
/// This option is intended for proving primality for integers that have already been checked using simpler methods.
/// For example one could generate random integers without small factors and then prove that they are prime faster than with 
/// is_prime_ac. Other applications include checking primality within a factorization function. 
/// Approximately 13% faster against primes than is_prime_ac
#[no_mangle]
pub extern "C" fn is_prime_wc(x: u64) -> bool { 

        if !sprp(x,2u64){
            return false;
        }

        let idx = ((x as u32).wrapping_mul(942441121)>>14) as usize;
        sprp(x,FERMAT_BASE[idx] as u64)
}   
    
