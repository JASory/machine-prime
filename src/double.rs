#[cfg(any(feature = "lucas", feature = "table", feature = "ssmr"))]
use crate::primes::{INV_8, PRIME_INV_64};

/// Multiplicative inverse over Z/2^128
///
///  In:  n \in 2Z + 1
///
/// Out: n^-1
pub const fn mul_inv2_128(n: u128) -> u128 {
    #[cfg(not(any(feature = "lucas", feature = "table", feature = "ssmr")))]
    {
        let mut est: u128 = 3u128.wrapping_mul(n) ^ 2;
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }

    #[cfg(any(feature = "lucas", feature = "table", feature = "ssmr"))]
    {
        let mut est: u128 = INV_8[((n >> 1) & 0x7F) as usize] as u128;
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est = 2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est);
        est
    }
}

#[inline(always)]
const fn split_to_u128(x: u128) -> (u128, u128) {
    (x.wrapping_shr(64), x & 0xFFFFFFFFFFFFFFFF)
}

/// Check if non-quadratic residue, 128-bit form
///
///  In: A,K
///
/// Out: Jacobi(A,K) == -1
pub const fn nqr_128(a: u128, k: u128) -> bool {
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

/// Lucas parameter search, 128-bit form
///
///  In: N
///
/// Out: x := jacobi(x^2-4,N) == -1
pub const fn param_search_128(n: u128) -> u128 {
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

    let mut d: u128;
    let mut p = 6u128;
    loop {
        d = p.wrapping_mul(p).wrapping_sub(4);
        if nqr_128(d, n) {
            break;
        }
        p = p.wrapping_add(1);
    }
    p
}

/// Product of two 128-bit integers
///
/// In: X,Y
/// 
/// Out: XY mod 2^128, XY/2^128
pub const fn u256prod(lhs: u128, rhs: u128) -> (u128, u128) {
    // hi,low
    let ((x1, x0), (y1, y0)) = (split_to_u128(lhs), split_to_u128(rhs));

    let z2 = x1.wrapping_mul(y1);
    let (c0, z0) = split_to_u128(x0.wrapping_mul(y0));
    let (c1, z1) = split_to_u128(x1.wrapping_mul(y0).wrapping_add(c0));
    let z2 = z2.wrapping_add(c1);
    let (c1, z1) = split_to_u128(x0.wrapping_mul(y1).wrapping_add(z1));

    (z2.wrapping_add(c1), z0 | z1.wrapping_shl(64)) // lo,hi returned
}

/// Lower product of two 128-bit integers
///
/// In: X,Y
/// 
/// Out: X*Y mod 2^128
pub const fn u256prod_lo(lhs: u128, rhs: u128) -> u128 {
    let ((x0, x1), (y0, y1)) = (split_to_u128(lhs), split_to_u128(rhs));

    let z2 = x0.wrapping_mul(y0);
    let c0 = x1.wrapping_mul(y1).wrapping_shr(64);
    let (c1, z1) = split_to_u128(x0.wrapping_mul(y1).wrapping_add(c0));
    let z2 = z2.wrapping_add(c1);
    let c1 = x1.wrapping_mul(y0).wrapping_add(z1).wrapping_shr(64);
    z2.wrapping_add(c1)
}

/// Square of a 128-bit integer
///
/// In: X 
///
/// Out:  X^2 mod 2^128, X^2 / 2^128
pub const fn u256sqr(x: u128) -> (u128, u128) {
    // hi,low
    let (x1, x0) = split_to_u128(x);

    let z2 = x1.wrapping_mul(x1);
    let m = x1.wrapping_mul(x0);
    let (c0, z0) = split_to_u128(x0.wrapping_mul(x0));
    let (c1, z1) = split_to_u128(m.wrapping_add(c0));
    let z2 = z2.wrapping_add(c1);
    let (c1, z1) = split_to_u128(m.wrapping_add(z1));
    (z2.wrapping_add(c1), z0 | z1.wrapping_shl(64)) // lo,hi 
}

/// One in Montgomery form, 128-bit form
///
/// In: N
///
/// Out: Mont(1,N)
#[inline]
pub const fn one_mont_128(n: u128) -> u128 {
    (u128::MAX % n).wrapping_add(1)
}

/// Two in Montgomery form, 128-bit form
///
/// In: Mont(1,N), N
///
/// Out: Mont(2,N)
pub const fn two_mont_128(one: u128, n: u128) -> u128 {
    let two = one.wrapping_add(one);
    if two > n {
        return two.wrapping_sub(n);
    }
    two
}

/// Subtraction in Montgomery form, 128-bit form
///   
/// In: X,Y,N
///  
/// Out: X-Y mod N
pub const fn mont_sub_128(x: u128, y: u128, n: u128) -> u128 {
    if y > x {
        return n.wrapping_sub(y.wrapping_sub(x));
    }
    x.wrapping_sub(y)
}

/// Convert to Montgomery form, 128-bit form
///
/// In: X, N
///
/// Out: Mont(X,N)
pub const fn to_mont_128(x: u128, n: u128) -> u128 {
    const RADIX: u128 = 0x10000000000000000;

    let mut dividend = x;
    let mut divisor = n;

    let s = divisor.leading_zeros();
    // Scale the values
    dividend = dividend.wrapping_shl(s);
    divisor = divisor.wrapping_shl(s);

    let (d1, d0) = split_to_u128(divisor);

    let (mut q1, mut rhat) = (dividend / d1, dividend % d1);

    let mut prod = q1.wrapping_mul(d0);
    let addend = RADIX.wrapping_mul(d1);
    let mut prod2 = RADIX.wrapping_mul(rhat);

    while q1 >= RADIX || prod > prod2 {
        q1 = q1.wrapping_sub(1);
        prod = prod.wrapping_sub(d0);
        rhat = rhat.wrapping_add(d1);
        prod2 = prod2.wrapping_add(addend);
        if rhat >= RADIX {
            break;
        }
    }

    let r21 = dividend
        .wrapping_mul(RADIX)
        .wrapping_sub(q1.wrapping_mul(divisor));

    let (mut q0, mut rhat) = (r21 / d1, r21 % d1);

    let mut prod = q0.wrapping_mul(d0);

    while q0 >= RADIX || prod > RADIX.wrapping_mul(rhat) {
        q0 = q0.wrapping_sub(1);
        rhat = rhat.wrapping_add(d1);
        prod = prod.wrapping_sub(d0);
        if rhat >= RADIX {
            break;
        }
    }

    let r = (r21
        .wrapping_mul(RADIX)
        .wrapping_sub(q0.wrapping_mul(divisor)))
        .wrapping_shr(s);
    r
}

/// Product in Montgomery form, 128-bit form
///
/// In: Mont(X,N),Mont(Y,N), N, N^-1
///
/// Out: Mont(XY,N)
pub const fn mont_prod_128(x: u128, y: u128, n: u128, npi: u128) -> u128 {
    let (hi, lo) = u256prod(x, y);
    let lo = lo.wrapping_mul(npi);
    let lo = u256prod_lo(lo, n);

    if hi < lo {
        hi.wrapping_sub(lo).wrapping_add(n)
    } else {
        hi.wrapping_sub(lo)
    }
}


/// Squaring in Montgomery form, 128-bit form
///
/// In: Mont(X,N), N, N^-1
///
/// Out: Mont(X^2,N)
pub const fn mont_sqr_128(x: u128, n: u128, npi: u128) -> u128 {
    let (hi, lo) = u256sqr(x);
    let lo = lo.wrapping_mul(npi);
    let lo = u256prod_lo(lo, n);

    if hi < lo {
        hi.wrapping_sub(lo).wrapping_add(n)
    } else {
        hi.wrapping_sub(lo)
    }
}

/// Modular exponentiation in Montgomery form, 128-bit form
///
///  In: Mont(base),Mont(1),pow,n, inv
///
/// Out: base^pow mod n
pub const fn mont_pow_128(mut base: u128, mut one: u128, mut p: u128, n: u128, npi: u128) -> u128 {
    while p > 1 {
        if p & 1 == 0 {
            base = mont_sqr_128(base, n, npi);
            p = p.wrapping_shr(1);
        } else {
            one = mont_prod_128(base, one, n, npi);
            base = mont_sqr_128(base, n, npi);
            p = p.wrapping_sub(1).wrapping_shr(1)
        }
    }
    mont_prod_128(base, one, n, npi)
}

///  Lucas-V sequence test with Selfridge parameters
/// 
/// In: N,Mont(1,N), Mont(2,N), N^-1
///
/// Out: Lucas_V(n)
pub const fn lucas_128(n: u128, one: u128, two: u128, npi: u128) -> bool {
    let n_plus = n.wrapping_add(1);
    let s = n_plus.trailing_zeros();
    let d = n_plus.wrapping_shr(s);

    let param = param_search_128(n);
    // Montgomery forms of starting parameter, and n-2
    let m_param = to_mont_128(param, n);

    let m_2_inv = mont_prod_128(mont_sub_128(n, two, n), one, n, npi);

    let mut w = mont_sub_128(mont_sqr_128(m_param, n, npi), two, n);
    let mut v = m_param;

    let b = 128u32.wrapping_sub(d.leading_zeros());

    let mut i = 2;

    while i < (b.wrapping_add(1)) {
        let t = mont_sub_128(mont_prod_128(v, w, n, npi), m_param, n);

        if d.wrapping_shr(b.wrapping_sub(i)) & 1 == 1 {
            v = t;
            w = mont_sub_128(mont_sqr_128(w, n, npi), two, n);
        } else {
            w = t;
            v = mont_sub_128(mont_sqr_128(v, n, npi), two, n);
        }
        i = i.wrapping_add(1);
    }

    if v == two || v == m_2_inv {
        return true;
    }

    let mut counter = 1;

    while counter < s {
        if v == 0 {
            return true;
        }
        v = mont_sub_128(mont_sqr_128(v, n, npi), two, n);
        if v == two {
            return false;
        }
        counter = counter.wrapping_add(1);
    }
    false
}

/// Strong Fermat test, 128-bit form
///
/// In: N,tz := a*2^tz+1 =N, Mont(base,N), Mont(1,N), Mont(N-1,N),
///
/// Out: SPRP(N,base)
pub const fn strong_fermat_128(
    p: u128,
    tz: u32,
    base: u128,
    one: u128,
    oneinv: u128,
    inv: u128,
) -> bool {
    let d = p.wrapping_sub(1).wrapping_shr(tz);

    let mut result = mont_pow_128(base, one, d, p, inv);

    if result == one || result == oneinv {
        return true;
    }

    let mut count = 1;

    while count < tz {
        count = count.wrapping_add(1);
        result = mont_sqr_128(result, p, inv);

        if result == oneinv {
            return true;
        }
    }
    false
}


const fn core_primality_128(x: u128) -> bool {
    let inv = mul_inv2_128(x);

    let tzc = x.wrapping_sub(1).trailing_zeros();
    let one = one_mont_128(x);
    let oneinv = mont_prod_128(mont_sub_128(x, one, x), one, x, inv);
    let two = two_mont_128(one, x);

    if !strong_fermat_128(x, tzc, two, one, oneinv, inv) {
        return false;
    }
    /* 
     Inconsequential optimisation
    if x < 0x10002400000000000{
       let base = to_mont_128(552491497,x);
         return strong_fermat_128(x, tzc, base, one, oneinv, inv);
    }
    */
    lucas_128(x, one, two, inv)
}

/// 128-bit is_prime_wc
///
/// Branches to use is_prime_wc for n < 2^64
/// No additional known errors
#[no_mangle]
pub const extern "C" fn is_prime_wc_128(x: u128) -> bool {
    if x < 0x10000000000000000{
       return crate::check::is_prime_wc(x as u64);
    }
    core_primality_128(x)
}

/// 128-bit is_prime
#[no_mangle]
pub const extern "C" fn is_prime_128(x: u128) -> bool {
    if x < 0x10000000000000000{
       return crate::check::is_prime(x as u64);
    }
    if x & 1 == 0 {
        return false;
    }

    #[cfg(any(feature = "lucas", feature = "table", feature = "ssmr"))]
    {
        let residue = (x % 13082761331670030u128) as u64;

        let mut idx: usize = 0;

        while idx < 13 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 10575651537777253u128) as u64;

        while idx < 21 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1)
        }

        let residue = (x % 9823972789433423u128) as u64;

        while idx < 29 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 805474958639317u128) as u64;

        while idx < 35 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 4575249731290429u128) as u64;

        while idx < 42 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 18506541671175721u128) as u64;

        while idx < 49 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 61247129307885343u128) as u64;

        while idx < 56 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 536967265590991u128) as u64;

        while idx < 62 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }

        let residue = (x % 3442087319857u128) as u64;

        while idx < 66 {
            if residue.wrapping_mul(PRIME_INV_64[idx]) < residue {
                return false;
            }
            idx = idx.wrapping_add(1);
        }
    } // end conditional block
    core_primality_128(x)
}
