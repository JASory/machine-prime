#[cfg(any(feature = "lucas", feature = "ssmr"))]
use crate::primes::PRIME_TABLE_128;

#[cfg(feature="qft")]
use crate::qft::qft;

use crate::check::mul_inv2;

/// Multiplicative inverse over Z/2^128
///
///  In:  n \in 2Z + 1
///
/// Out: n^-1
pub const fn mul_inv2_128(n: u128) -> u128 {
       let est : u128 = mul_inv2(n as u64) as u128;
       2u128.wrapping_sub(est.wrapping_mul(n)).wrapping_mul(est)
}

// In: X
// Out: X/2^64 X mod 2^64
#[inline(always)]
const fn split_to_u128(x: u128) -> (u128, u128) {
    (x>>64, (x as u64) as u128)
}

/// Check if non-quadratic residue, 128-bit form
///
///  In: A,K
///
/// Out: Jacobi(A,K) == -1
pub const fn nqr_128(a: u128, n: u128) -> bool {
     // The final sign is bit 1 of this variable.
     // The other 31 bits are garbage that are masked at the end.
    let mut sign: u32 = 0;

    let (mut a64, mut n64) = if n >> 64 == 0 {
        (a as u64, n as u64)
    } else {
        // We need one iteration with 128-bit types.
        if n == 0 {
            return false;
        }
        // Remove powers of 2 from a
        let zeros = a.trailing_zeros();
        let a = a >> zeros;
        // Flip sign if p == 3 or 5 (mod 8) and zeros is odd
        sign ^= (n as u32).wrapping_add(2) >> 1 & zeros << 1;

        // Quadratic recoprocity: Flip sign if p == n == 3 (mod 8)
        sign ^= a as u32 & n as u32;
        // This ensures both values fit into 64 bits
        ((n % a) as u64, a as u64)
    };
    
     while a64 != 0 {
         let zeros = a64.trailing_zeros();
         a64 >>= zeros;
         sign ^= (n64 as u32).wrapping_add(2) >> 1 & zeros << 1;
         sign ^= (a64 & n64) as u32;
        (a64, n64) = (n64 % a64, a64)
     }
     n64 == 1 && sign & 2 != 0
  }   

/// Lucas parameter search, 128-bit form
///
///  In: N
///
/// Out: x := jacobi(x^2-4,N) == -1
#[cfg(not(feature="qft"))]
pub const fn param_search_128(n: u128) -> u128 {
   // Short-cut the loop, these cases comprise the majority of inputs
    
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
    
 
    let mut p = 6u128;

    loop {
 
        if nqr_128(p*p-4, n) {
            break;
        }
        p +=1;
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

    let (c, z0) = split_to_u128(x0 * y0);
    let (c, z1) = split_to_u128(x1 * y0 + c);
    let z2 = x1 * y1 + c;
    let (c, z1) = split_to_u128(x0 * y1 + z1);
    
    (z2 + c, z0 | z1 << 64) // hi,lo returned
}

/// Higher product of two 128-bit integers
///
/// In: X,Y
/// 
/// Out: X*Y mod 2^128
pub const fn u256prod_hi(lhs: u128, rhs: u128) -> u128 {
    let ((x1, x0), (y1, y0)) = (split_to_u128(lhs), split_to_u128(rhs));
    let c = (x0 * y0) >> 64;
    let (c, z1) = split_to_u128(x1 * y0 + c);
    let z2 = x1 * y1 + c;
    let c = (x0 * y1 + z1) >> 64;
    z2 + c
}

/// Square of a 128-bit integer
///
/// In: X 
///
/// Out:  X^2 mod 2^128, X^2 / 2^128
pub const fn u256sqr(x: u128) -> (u128, u128) {
    // hi,lo
    let (x1, x0) = split_to_u128(x);

    let z2 = x1*x1;
    let m = x1*x0;
    let (c0, z0) = split_to_u128(x0*x0);
    let (c1, z1) = split_to_u128(m+c0);
    let z2 = z2+c1;
    let (c1, z1) = split_to_u128(m+z1);
    (z2.wrapping_add(c1), z0 | z1.wrapping_shl(64)) // hi,lo 
}

/// One in Montgomery form, 128-bit form
///
/// In: N
///
/// Out: Mont(1,N)
#[inline]
pub const fn one_mont_128(n: u128) -> u128 {
     n.wrapping_neg() % n
}

/// Two in Montgomery form, 128-bit form
///
/// In: Mont(1,N), N
///
/// Out: Mont(2,N)
pub const fn two_mont_128(one: u128, n: u128) -> u128 {
    let two = 2*one;
    if two >= n {
        return two-n;
    }
    two
}

/// Subtraction in Montgomery form, 128-bit form
///   
/// In: X,Y,N
///  
/// Out: X-Y mod N
pub const fn mont_sub_128(x: u128, y: u128, n: u128) -> u128 {
    if x >= y {
        x-y
    } else {
       x.wrapping_sub(y).wrapping_add(n)
    }
}

/// Written by David Sparks 
/// Convert to Montgomery form, 128-bit form
///
/// In: X, N where X < N
///
/// Out: Mont(X,N)
pub const fn to_mont_128(mut x: u128, n: u128) -> u128 {
    // Normalize the divisor so its msbit is set.
    debug_assert!(x < n);
    let s = n.leading_zeros();
    let divisor = n << s;
    x <<= s;
    debug_assert!(x < divisor);
 
     let (d1, d0) = split_to_u128(divisor);
    // The body of this loop computes x = ((x as u192) << 64) % divisor.
    // Repeating it twice achieves the desired computation.
    // Because the low half of the 256-bit dividend is zero, there's no
    // need to shift in additional low-order words, and we discard the
    // quotient, so the two iterations are identical.
    let mut i = 0;
    while i < 2 {
        i += 1;
        if x >> 64 >= d1 {
            // Exception path: avoid having q > u64::MAX, even temporarily.
            let q = u64::MAX;
            x = (x << 64).wrapping_sub(q as u128 * divisor);
            continue;
         }

        // Normal path: estimate quotient digit.  Maybe high, never low.
        let (q, mut r) = ((x / d1) as u64, (x % d1) as u64);
        let mut prod = q as u128 * d0;
        // Now correct the quotient digit to include d0.
        // (This loops at most twice, but more than one iteration is
        // so rare that it's not worth unrolling; the slight gain from
        // avoiding the test for a third iteration is negligible.)
        while (prod >> 64) as u64 > r {
            //q -=  1;
            prod -= d0;
            let carry;
            (r, carry) = r.overflowing_add(d1 as u64);
            if carry {
                break;
            }
        }
        // x = (x << 64).wrapping_sub(q * divisor);
        // But since we already have r = x - q * d1 and prod = q * d0,
        // we can optimize it a little, and avoid even keeping track of
        // q explicitly.  r might have overflowed 64 bits, but this will
        // wrap it back to the correct range.
        x = ((r as u128) << 64).wrapping_sub(prod);
     }
     x >> s

}

/// Product in Montgomery form, 128-bit form
///
/// In: Mont(X,N),Mont(Y,N), N^-1, N
///
/// Out: Mont(XY,N)
pub const fn mont_prod_128(x: u128, y: u128, inv: u128, n: u128) -> u128 {
    let (hi, lo) = u256prod(x, y);
    let lo = lo.wrapping_mul(inv);
    let carry = u256prod_hi(lo, n);

    if hi < carry {
        hi.wrapping_sub(carry).wrapping_add(n)
    } else {
        hi.wrapping_sub(carry)
    }
}


/// Squaring in Montgomery form, 128-bit form
///
/// In: Mont(X,N), N^-1, N,
///
/// Out: Mont(X^2,N)
pub const fn mont_sqr_128(x: u128, inv: u128, n: u128) -> u128 {
    let (hi, lo) = u256sqr(x);
    let lo = lo.wrapping_mul(inv);
    let carry = u256prod_hi(lo, n);

    if hi < carry {
        hi.wrapping_sub(carry).wrapping_add(n)
    } else {
        hi.wrapping_sub(carry)
    }
}

/// Modular exponentiation in Montgomery form, 128-bit form
///
///  In: Mont(base),Mont(1),pow,n, inv
///
/// Out: base^pow mod n
pub const fn mont_pow_128(mut base: u128, mut one: u128, mut p: u128, inv: u128, n: u128) -> u128 {
    while p > 1 {
        if p & 1 == 0 {
            base = mont_sqr_128(base, inv,n);
            p >>=1;
        } else {
            one = mont_prod_128(one, base, inv,n);
            base = mont_sqr_128(base, inv,n);
            p >>=1;
        }
    }
    mont_prod_128(one,base, inv, n)
}

///  Lucas-V sequence test with Selfridge parameters
/// 
/// In: N,Mont(1,N), Mont(2,N), N^-1
///
/// Out: Lucas_V(n)
#[cfg(not(feature="qft"))]
pub const fn lucas_128(n: u128, one: u128, two: u128, inv: u128) -> bool {
    // 2^128-1 is not a base-2 pseudoprime so overflow will never happen
    let n_plus = n+1;
    let s = n_plus.trailing_zeros();
    let d = n_plus>>s;

    let param = param_search_128(n);
    // Montgomery forms of starting parameter, and n-2
    let m_param = to_mont_128(param, n);

    let m_2_inv = mont_prod_128(mont_sub_128(n, two, n), one, inv, n);

    let mut w = mont_sub_128(mont_sqr_128(m_param, inv, n), two, n);
    let mut v = m_param;

    let b : u32 = 128-d.leading_zeros();

    let mut i = 2;

    while i < (b+1) {
        let t = mont_sub_128(mont_prod_128(v, w, inv, n), m_param, n);

        if (d>>(b-i)) & 1 == 1 {
            v = t;
            w = mont_sub_128(mont_sqr_128(w, inv, n), two, n);
        } else {
            w = t;
            v = mont_sub_128(mont_sqr_128(v, inv, n), two, n);
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
        v = mont_sub_128(mont_sqr_128(v, inv, n), two, n);
        if v == two {
            return false;
        }
        counter +=1;
    }
    false
}

/// Strong Fermat test, 128-bit form
///
/// In: N,tz := a*2^tz+1 =N, Mont(base,N), Mont(1,N), Mont(N-1,N), N^-1
///
/// Out: SPRP(N,base)
pub const fn strong_fermat_128(
    n: u128,
    tz: u32,
    base: u128,
    one: u128,
    oneinv: u128,
    inv: u128,
) -> bool {
    let d = n>>tz;

    let mut result = mont_pow_128(base, one, d, inv, n);

    if result == one || result == oneinv {
        return true;
    }

    let mut count = 1;

    while count < tz {
        count +=1;
        result = mont_sqr_128(result, inv, n);

        if result == oneinv {
            return true;
        }
    }
    false
}


const fn core_primality_128(x: u128) -> bool {
    let inv = mul_inv2_128(x);

    let tzc = (x-1).trailing_zeros();
    let one = one_mont_128(x);
    let oneinv = x.wrapping_sub(one);
    let two = two_mont_128(one, x);
    
    if !strong_fermat_128(x, tzc, two, one, oneinv, inv) {
        return false;
    }
    
    let sqrt = x.isqrt();
    // Guarantees that the search for a nonquadratic residue will succeed
    // This is unnecessary if there does not exist a weiferich prime between 2^32 and 2^64
    // Which is probably the case. see Dorais and Klyve. 
    if sqrt*sqrt == x{
      return false;
    }
    /* 
     Inconsequential optimisation, kept here for novelty really
    if x < 0x10002400000000000{
       let base = to_mont_128(552491497,x);
         return strong_fermat_128(x, tzc, base, one, oneinv, inv);
    }
    */
    
    // Calls Khashin's QFT 
    // Strictly speaking simply calling this by itself would be faster for the worst-case
    // However, the base-2 strong fermat test is substantially faster so it's not terribly impactful
    // to the total runtime. See qft.rs for discussion of why we add a base-2 strong fermat
    #[cfg(feature="qft")]
    {
      qft(x,one,two,oneinv,inv)
    }
    #[cfg(not(feature="qft"))]
    {
    lucas_128(x, one, two, inv)
    }
}

/// 128-bit is_prime_wc
///
/// Branches to use is_prime_wc for n < 2^64
/// # Wide
/// No additional known errors, BPSW pseudoprimes may exist
/// # QFT 
/// No additional known errors, there may exist pseudoprimes to both the quadratic frobenius test and base-2 fermat
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

    #[cfg(any(feature = "lucas", feature = "ssmr"))]
    {
    
    let mut idx: usize = 0;
        
        while idx < 256 {
        
          let prod = x.wrapping_mul(PRIME_TABLE_128[idx]);
          
          if prod <= PRIME_TABLE_128[idx+1]{
             return prod==1;
          }
           idx +=2;
        }   
    } // end conditional block
    core_primality_128(x)
}
