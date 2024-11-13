#[cfg(not(any(feature = "small", feature = "tiny")))]
use crate::hashbase::FERMAT_BASE;

#[cfg(not(feature = "tiny"))]
use crate::primes::{INV_8, PRIME_INV_64};

#[cfg(feature = "small")]
use crate::primes::LUCAS_PARAM;

//  In:  n \in 2Z + 1
// Out: n^-1
const fn mod_inv(n: u64) -> u64 {
    #[cfg(feature = "tiny")]
    {
        let mut est: u64 = 3u64.wrapping_mul(n) ^ 2;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }

    #[cfg(not(feature = "tiny"))]
    {
        let mut est: u64 = INV_8[((n >> 1) & 0x7F) as usize] as u64;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }
}

//
const fn mont_prod(x: u64, y: u64, n: u64, npi: u64) -> u64 {
    let interim: u128 = (x as u128).wrapping_mul(y as u128);
    let lo: u64 = (interim as u64).wrapping_mul(npi);
    let lo: u64 = ((lo as u128).wrapping_mul(n as u128) >> 64) as u64;
    let hi: u64 = (interim >> 64) as u64;

    if hi < lo {
        hi.wrapping_sub(lo).wrapping_add(n)
    } else {
        hi.wrapping_sub(lo)
    }
}

const fn to_z(x: u64, n: u64, npi: u64) -> u64 {
    let lo: u64 = x.wrapping_mul(npi);
    let lo: u64 = ((lo as u128).wrapping_mul(n as u128) >> 64) as u64;

    lo.wrapping_neg().wrapping_add(n)
}

// In: X,Y,N
// Out: X-Y mod N
const fn mont_sub(x: u64, y: u64, n: u64) -> u64 {
    if y > x {
        return n.wrapping_sub(y.wrapping_sub(x));
    }
    x.wrapping_sub(y)
}

//  In: A,K
// Out: Jacobi(A,K) == -1
#[cfg(any(feature = "small", feature = "tiny"))]
const fn jacobi(a: u64, k: u64) -> bool {
    let mut n = a;
    let mut p = k;
    let mut t = false;

    while n != 0 {
        let zeros = n.trailing_zeros();
        n >>= zeros;
           
        if (p&7 == 3 || p&7 == 5) && (zeros&1 == 1) {
            t^=true;
         }
        
        if p&3 == 3 && n&3 == 3 {
            t^=true;
        }

        let interim = p;
        p = n;
        n = interim;

        n %= p;
    }

    if p == 1 {
        t
    } else {
        false
    }
}

//  In: N
// Out: x := jacobi(x*x-4,N) == -1
#[cfg(any(feature = "small", feature = "tiny"))]
const fn param_search(n: u64) -> u64 {
    #[cfg(feature = "small")]
    {
        let rem = n % 5;

        if rem == 3 || rem == 2 {
            return 3;
        }

        let rem = n % 12;

        if rem == 2 || rem == 5 || rem == 7 || rem == 8 {
            return 4;
        }

        let rem = n % 21;

        if rem == 2 || rem == 8 || rem == 11 || rem == 10 || rem == 13 || rem == 19 {
            return 5;
        }

        let mut idx: usize = 0;

        while idx < 27 {
            let i = LUCAS_PARAM[idx] as u64;
            if jacobi(i.wrapping_mul(i).wrapping_sub(4u64), n){
                return i;
            }
            idx += 1;
        }
        0u64
    }

    #[cfg(feature = "tiny")]
    {
        let mut d: u64;
        let mut p = 3u64;
        loop {
            d = p.wrapping_mul(p).wrapping_sub(4);
            if jacobi(d,n){
                break;
            }
            p += 1;
        }
        p
    }
}

// Only Small and Tiny modes use Lucas sequence test
// Perfect squares that fail 1194649, 12327121
#[cfg(any(feature = "small", feature = "tiny"))]
const fn lucas(n: u64, one: u64, npi: u64) -> bool {
    let param = param_search(n);

    #[cfg(feature = "small")]
    {
        if param == 0 {
            return true;
        }
    }
    
    let n_plus = n.wrapping_add(1);
    let s = n_plus.trailing_zeros();
    let d = n_plus.wrapping_shr(s);
     // Almost works

   // let m_param = mont_prod(param.wrapping_mul(one), one, n, npi);
   // let two = one.wrapping_add(one);
   // let m_2 = mont_prod(two, one, n, npi);
   // let m_2_inv = mont_prod(mont_sub(n, two, n), one, n, npi);
    // Montgomery forms of starting parameter, 2, and n-2
    let m_param = (((param as u128) << 64) % (n as u128)) as u64;
    let m_2 = ((2u128 << 64) % (n as u128)) as u64;
    let m_2_inv = (((n.wrapping_sub(2) as u128) << 64) % (n as u128)) as u64;
    
    let mut w = mont_sub(mont_prod(m_param, m_param, n, npi), m_2, n);
    let mut v = m_param;

    let b = 64u32.wrapping_sub(d.leading_zeros());

    let mut i = 2;

    while i < (b.wrapping_add(1)) {
        let t = mont_sub(mont_prod(v, w, n, npi), m_param, n);

        if (d >> (b - i)) & 1 == 1 {
            v = t;
            w = mont_sub(mont_prod(w, w, n, npi), m_2, n);
        } else {
            w = t;
            v = mont_sub(mont_prod(v, v, n, npi), m_2, n);
        }
        i += 1;
    }

    if v == m_2 || v == m_2_inv {
        return true;
    }

    let mut counter = 1;

    while counter < s {
        if v == 0 {
            return true;
        }
        v = mont_sub(mont_prod(v, v, n, npi), m_2, n);
        if v == m_2 {
            return false;
        }
        counter += 1;
    }
    false
}

//  In: 
// Out:

const fn mont_pow(mut base: u64, mut one: u64, mut p: u64, n: u64, npi: u64) -> u64 {
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
const fn euler_p(p: u64, one: u64, npi: u64) -> bool {
    let res = p & 7;
    
    let mut param : u32 = 0;

    if res == 1 {
        param = 1;
    }
    // Montgomery form of 2
    let mut result = one.wrapping_add(one);
    // p-1
    let p_minus = p.wrapping_sub(1);
    
    let d = p_minus.wrapping_shr(param.wrapping_add(1));

    result = mont_pow(result, one, d, p, npi);

    result = to_z(result, p, npi);

    if result == 1 {
        return res == 1 || res == 7;
    } else if result == p_minus {
        return res == 1 || res == 3 || res == 5;
    }
    false
}

// Only the Default and SSMR modes call this function
#[cfg(not(any(feature = "small", feature = "tiny")))]
const fn sprp(p: u64, base: u64, one: u64, npi: u64) -> bool {

    let p_minus = p.wrapping_sub(1);
    let twofactor = p_minus.trailing_zeros();
    let d = p_minus >> twofactor;

    let mut result = (((base as u128) << 64) % (p as u128)) as u64;

    let oneinv = mont_prod(mont_sub(p, one, p), one, p, npi);

    result = mont_pow(result, one, d, p, npi);

    if result == one || result == oneinv {
        return true;
    }

    let mut count = 1;

    while count < twofactor {
        count += 1;
        result = mont_prod(result, result, p, npi);

        if result == oneinv {
            return true;
        }
    }
    false
}

const fn core_primality(x: u64) -> bool {
    let npi = mod_inv(x);
    let one = (u64::MAX % x).wrapping_add(1);

    #[cfg(feature = "ssmr")]
    {
        // Ordering of fermat tests is swapped as apparently there is a notable cache penalty
        // if you check the size of the integer first, so we just run the fermat test immediately
        // This variant has nearly identical performance to the default, although the default in theory should be marginally faster
        // In practice this is probably the better option
        let idx = (x as u32).wrapping_mul(2559736147).wrapping_shr(14) as usize;
        // Branch to a single shot primality test
        if !sprp(x, FERMAT_BASE[idx] as u64, one, npi) {
            return false;
        }
        // If greater than the first error -1
        if x > 1u64<<40 {
            return euler_p(x, one, npi);
        }
        return true;
    }

    #[cfg(not(any(feature = "small", feature = "tiny", feature = "ssmr")))]
    {
        if  !euler_p(x, one, npi) {
            return false;
        }
        let idx = (x as u32).wrapping_mul(2559736147).wrapping_shr(14) as usize;
        sprp(x, FERMAT_BASE[idx] as u64, one, npi)
    }

    #[cfg(any(feature = "small", feature = "tiny"))]
    {
        if !euler_p(x, one, npi) {
            return false;
        }

        if x < 1729 {
            return true;
        }

        lucas(x, one, npi)
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
        if x < 55730344633563600 {
            let mut idx: usize = 0;

            while idx < 66 {
                let prod = x.wrapping_mul(PRIME_INV_64[idx]);
                if prod == 1 {
                    return true;
                }
                if prod < x {
                    return false;
                }
                idx += 1;
            }
        }

        if x > 55730344633563600 {
            let interim = x % 13082761331670030u64;

            let mut idx: usize = 0;

            while idx < 13 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 10575651537777253u64;

            while idx < 21 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1
            }

            let interim = x % 9823972789433423u64;

            while idx < 29 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 805474958639317u64;

            while idx < 35 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 4575249731290429u64;

            while idx < 42 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 18506541671175721u64;

            while idx < 49 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 61247129307885343u64;

            while idx < 56 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 536967265590991u64;

            while idx < 62 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }

            let interim = x % 3442087319857u64;

            while idx < 66 {
                if interim.wrapping_mul(PRIME_INV_64[idx]) < interim {
                    return false;
                }
                idx += 1;
            }
        } //end if
        
    } // end conditional block

    core_primality(x)
}

/// Primality testing for the worst case. 
/// 
/// Panics at zero, flags 1 as prime, 2 as composite.
/// # SSMR
/// Flags the perfect powers of 2 as well as 10,38,5368709120 and 7516192768 as prime
/// # Small
/// "Erroneously" returns true for the perfect squares 1194649 (1093^2) and 12327121 (3511^2). This is due to slightly faster parameter selection
/// # Tiny
/// Infinitely loops at the perfect squares 1194649 and 12327121.
#[no_mangle]
pub const extern "C" fn is_prime_wc(x: u64) -> bool {
    /*
    Alerts for the failure points
    compiled library from Makefile does not have this check
    */
    debug_assert!(x != 1 && x != 2 && x != 0);
    #[cfg(feature="ssmr")]
    {
      debug_assert!(!x.is_power_of_two());
      debug_assert!(x != 10 && x != 38 && x != 5368709120 &&
x != 7516192768);
    }
    #[cfg(any(feature = "small", feature = "tiny"))]
    {
        debug_assert!(x != 1194649 && x != 12327121);
    }

    core_primality(x)
}
