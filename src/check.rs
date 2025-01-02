#[cfg(any(feature = "table", feature = "ssmr"))]
use crate::hashbase::FERMAT_TABLE;

#[cfg(any(feature = "lucas", feature = "table", feature = "ssmr"))]
use crate::primes::{INV_8, PRIME_INV_64};

#[cfg(all(feature = "lucas", not(any(feature = "table", feature = "ssmr"))))]
use crate::primes::LUCAS_PARAM;

#[cfg(feature = "wide")]
use crate::double::{is_prime_128, is_prime_wc_128};

/// Multiplicative inverse over Z/2^64
///
///  In:  n \in 2Z + 1
///
/// Out: n^-1
pub const fn mul_inv2(n: u64) -> u64 {
    #[cfg(not(any(feature = "lucas", feature = "table", feature = "ssmr")))]
    {
        let mut est: u64 = 3u64.wrapping_mul(n) ^ 2;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }

    #[cfg(any(feature = "lucas", feature = "table", feature = "ssmr"))]
    {
        let mut est: u64 = INV_8[((n >> 1) & 0x7F) as usize] as u64;
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u64.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }
}

/// Product in Montgomery form
///
/// In: Mont(X,N),Mont(Y,N), N, N^-1
///
/// Out: Mont(X*Y,N)

pub const fn mont_prod(x: u64, y: u64, n: u64, inv: u64) -> u64 {
    let full_prod: u128 = (x as u128).wrapping_mul(y as u128);
    let lo: u64 = (full_prod as u64).wrapping_mul(inv);
    let lo: u64 = ((lo as u128).wrapping_mul(n as u128) >> 64) as u64;
    let hi: u64 = (full_prod >> 64) as u64;

    if hi < lo {
        hi.wrapping_sub(lo).wrapping_add(n)
    } else {
        hi.wrapping_sub(lo)
    }
}

/// Convert to Montgomery form
///
/// In: X, N
///
/// Out: Mont(X,N)
#[inline]
pub const fn to_mont(x: u64, n: u64) -> u64 {
    (((x as u128) << 64) % (n as u128)) as u64
}

/// One in Montgomery form
///
/// In: N
///
/// Out: Mont(1,N)
#[inline]
pub const fn one_mont(n: u64) -> u64 {
    (u64::MAX % n).wrapping_add(1)
}

/// Two in Montgomery form
///
/// In: Mont(1,N), N
///
/// Out: Mont(2,N)
pub const fn two_mont(one: u64, n: u64) -> u64 {
    let two = one.wrapping_add(one);
    if two > n {
        return two.wrapping_sub(n);
    }
    two
}

/// Subtraction in Montgomery form   
///   
/// In: X,Y,N
///  
/// Out: X-Y mod N
pub const fn mont_sub(x: u64, y: u64, n: u64) -> u64 {
    if y > x {
        return n.wrapping_sub(y.wrapping_sub(x));
    }
    x.wrapping_sub(y)
}

/// Check if non-quadratic residue
///
///  In: A,K
///
/// Out: Jacobi(A,K) == -1
#[cfg(not(any(feature = "table", feature = "ssmr")))]
pub const fn nqr(a: u64, k: u64) -> bool {
    let mut n = a;
    let mut p = k;
    let mut t = false;

    while n != 0 {
        let zeros = n.trailing_zeros();
        n >>= zeros;

        if (p & 7 == 3 || p & 7 == 5) && (zeros & 1 == 1) {
            t ^= true;
        }

        if p & 3 == 3 && n & 3 == 3 {
            t ^= true;
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

/// Lucas parameter search
///
///  In: N
///
/// Out: x := jacobi(x*x-4,N) == -1
#[cfg(not(any(feature = "table", feature = "ssmr")))]
pub const fn param_search(n: u64) -> u64 {
    #[cfg(all(feature = "lucas", not(any(feature = "table", feature = "ssmr"))))]
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
            if nqr(i.wrapping_mul(i).wrapping_sub(4u64), n) {
                return i;
            }
            idx += 1;
        }
        0u64
    }

    #[cfg(not(any(feature = "lucas", feature = "table", feature = "ssmr")))]
    {
        let mut d: u64;
        let mut p = 3u64;
        loop {
            d = p.wrapping_mul(p).wrapping_sub(4);
            if nqr(d, n) {
                break;
            }
            p += 1;
        }
        p
    }
}

///  Lucas-V sequence test with Selfridge parameters
/// 
/// In: N,Mont(1,N), Mont(2,N), N^-1
///
/// Out: Lucas_V(n)
#[cfg(not(any(feature = "table", feature = "ssmr")))]
pub const fn lucas(n: u64, one: u64, two: u64, npi: u64) -> bool {
    let param = param_search(n);

    #[cfg(all(feature = "lucas", not(any(feature = "table", feature = "ssmr"))))]
    {
        if param == 0 {
            return true;
        }
    }

    let n_plus = n.wrapping_add(1);
    let s = n_plus.trailing_zeros();
    let d = n_plus.wrapping_shr(s);

    let m_param = to_mont(param, n);

    let m_2_inv = mont_prod(mont_sub(n, two, n), one, n, npi);

    let mut w = mont_sub(mont_prod(m_param, m_param, n, npi), two, n);
    let mut v = m_param;

    let b = 64u32.wrapping_sub(d.leading_zeros());

    let mut i = 2;

    while i < (b.wrapping_add(1)) {
        let t = mont_sub(mont_prod(v, w, n, npi), m_param, n);

        if (d >> (b - i)) & 1 == 1 {
            v = t;
            w = mont_sub(mont_prod(w, w, n, npi), two, n);
        } else {
            w = t;
            v = mont_sub(mont_prod(v, v, n, npi), two, n);
        }
        i += 1;
    }

    if v == two || v == m_2_inv {
        return true;
    }

    let mut counter = 1;

    while counter < s {
        if v == 0 {
            return true;
        }
        v = mont_sub(mont_prod(v, v, n, npi), two, n);
        if v == two {
            return false;
        }
        counter += 1;
    }
    false
}

/// Modular exponentiation in Montgomery form
///
///  In: Mont(base),Mont(1),pow,n, inv
///
/// Out: base^pow mod n
pub const fn mont_pow(mut base: u64, mut one: u64, mut pow: u64, n: u64, npi: u64) -> u64 {
    while pow > 1 {
        if pow & 1 == 0 {
            base = mont_prod(base, base, n, npi);
            pow >>= 1;
        } else {
            one = mont_prod(one, base, n, npi);
            base = mont_prod(base, base, n, npi);
            pow = (pow - 1) >> 1;
        }
    }
    mont_prod(one, base, n, npi)
}

/// Fermat base selection for n < 2^64
#[cfg(any(feature = "table", feature = "ssmr"))]
#[inline]
pub const fn base_selector(x: u64) -> u64 {
    FERMAT_TABLE[(x as u32).wrapping_mul(811484239).wrapping_shr(14) as usize] as u64
}

/// Strong Fermat test
///
/// In: N,tz := a*2^tz+1 =N, Mont(base,N), Mont(1,N), Mont(N-1,N),
///
/// Out: SPRP(N,base)
pub const fn strong_fermat(n: u64, tz: u32, base: u64, one: u64, oneinv: u64, inv: u64) -> bool {
    let d = n.wrapping_sub(1) >> tz;

    let mut result = mont_pow(base, one, d, n, inv);

    if result == one || result == oneinv {
        return true;
    }

    let mut count = 1;

    while count < tz {
        count += 1;
        result = mont_prod(result, result, n, inv);

        if result == oneinv {
            return true;
        }
    }
    false
}

const fn core_primality(x: u64) -> bool {
    let inv = mul_inv2(x);

    let tzc = x.wrapping_sub(1).trailing_zeros();
    let one = one_mont(x);
    let oneinv = mont_prod(mont_sub(x, one, x), one, x, inv);

    #[cfg(feature = "ssmr")]
    {
        let base = base_selector(x);
        if !strong_fermat(x, tzc, to_mont(base, x), one, oneinv, inv) {
            return false;
        }

        if x > 1u64 << 47 {
            let two = two_mont(one, x);
            return strong_fermat(x, tzc, two, one, oneinv, inv);
        }
        return true;
    }

    #[cfg(all(feature = "table", not(feature = "ssmr")))]
    {
        let two = two_mont(one, x);
        if !strong_fermat(x, tzc, two, one, oneinv, inv) {
            return false;
        }

        let base = base_selector(x);
        strong_fermat(x, tzc, to_mont(base, x), one, oneinv, inv)
    }

    #[cfg(not(any(feature = "table", feature = "ssmr")))]
    {
        let two = two_mont(one, x);
        if !strong_fermat(x, tzc, two, one, oneinv, inv) {
            return false;
        }

        if x < 2047 {
            return true;
        }

        lucas(x, one, two, inv)
    }
}

/// Primality testing optimized for the average case in the interval 0;2^64.
///
/// Approximately 5 times faster than is_prime_wc in the average case, but slightly slower in the worst case.
pub const extern "C" fn is_prime(x: u64) -> bool {
    if x == 1 {
        return false;
    }

    #[cfg(not(any(feature = "table", feature = "ssmr")))]
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

    #[cfg(any(feature = "lucas", feature = "table", feature = "ssmr"))]
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
            let residue = x % 13082761331670030u64;

            let mut idx: usize = 0;

            while idx < 13 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 10575651537777253u64;

            while idx < 21 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1
            }

            let residue = x % 9823972789433423u64;

            while idx < 29 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 805474958639317u64;

            while idx < 35 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 4575249731290429u64;

            while idx < 42 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 18506541671175721u64;

            while idx < 49 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 61247129307885343u64;

            while idx < 56 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 536967265590991u64;

            while idx < 62 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                    return false;
                }
                idx += 1;
            }

            let residue = x % 3442087319857u64;

            while idx < 66 {
                if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
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
/// May pass some even numbers as prime
/// # Lucas
/// "Erroneously" returns true for the perfect squares 1194649 (1093^2) and 12327121 (3511^2). This is due to slightly faster parameter selection
/// # Tiny
/// Infinitely loops at the perfect squares 1194649 and 12327121.
/// # Wide
/// No additional known errors
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
    #[cfg(not(any(feature = "table", feature = "ssmr")))]
    {
        debug_assert!(x != 1194649 && x != 12327121);
    }

    core_primality(x)
}
