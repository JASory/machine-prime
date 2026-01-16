use crate::wide::{to_mont_128,mont_prod_128,mont_sqr_128, nqr_128, mont_sub_128};
/*
  Sergei Khashin's Frobenius primality test, described in Evaluation of the Effectiveness of the Frobenius Primality Test
  
  let c be the minimum integer in the sequence [n-1,2,3,4,5,..] such that Jacobi(c,n)=-1 e.g c is a nonquadratic residue of n
  
  then n is prime iff 
   
   Case c == n-1
       (2 + sqrt(-1))^n mod n = 2 + -sqrt(-1)
   Case c == 2
       (2 + sqrt(2))^n mod n= 2 + -sqrt(2)
   Case c > 2
      (1 + sqrt(c))^n = 1-sqrt(c)
  
  This is a bit slower than the BPSW but is potentially much stronger if not foolproof
  
  Khashin notes that a Frobenius pseudoprime with parameters (a,b,c) is also a fermat pseudoprime to a^2-b^2*c
  
  Therefore a Frobenius pseudoprime to each case must also be a fermat pseudoprime to a witness 
  
   Case c == n-1
     2^2-1*-1 = 5
   case c == 2 
     2^2-1*2 = 2
   Case c > 2
     1+-1c  = n-c+1
 
Note the converse is not true. The QFT is far stronger than the fermat test, this implementation has no known counterexamples. Khashin evaluated it up to 2^64. And I ran it against the 59043 pseudoprimes to the first 15 primes that are under 2^128

Currently is_prime_wc_128 employs a base-2 strong pseudoprime, prior to the QFT. This is for 2 reasons

  1. It eliminates most composites and approximately 70% of the remaining integers will be evaluated more quickly in the first 2 cases
  2. Base-2 strong fermat is particularly fast to evaluate, and the QFT is particularly slow so adding it on top reduces the
     probability of error, which is the purpose of this option.
  

  Unfortunately using witness of 2 is not particularly strong, it only substantially benefits the first and third case.
  
  Good candidates for a different strong fermat witness are 72,15,37,60,9375
*/

// In: X < N
// Out: X+X mod N
const fn double(x: u128, n: u128) -> u128{
   let sum = x.wrapping_shl(1);
   if sum >= n || sum < x{
     return sum.wrapping_sub(n)
   }
  sum
}

// In: X,Y < N
// Out: X+Y mod N
const fn mont_add(x: u128, y: u128, n: u128) -> u128{
   let sum = x.wrapping_add(y);
   if sum >= n || sum < x{
     return sum.wrapping_sub(n)
   }
  sum
}

// 2*x*y
const fn sqr_coef(x: u128, y: u128, inv: u128, n: u128) -> u128{
   double(mont_prod_128(x,y,inv,n),n)
}

// x,y * a,b
const fn prod_coef(x: u128, y: u128, a: u128, b: u128, inv: u128, n: u128) -> u128{
   mont_add(mont_prod_128(x,b,inv,n),mont_prod_128(y,a,inv,n),n)
}

// Gaussian Integer arithmetic case a+bi where i = sqrt(-1)

// x^2 - y^2
const fn gaussian_sqr_real(x: u128, y: u128, inv: u128, n: u128) -> u128{
   mont_sub_128(mont_sqr_128(x,inv,n),mont_sqr_128(y,inv,n),n)
}

const fn gaussian_prod_real(x: u128, y: u128, a: u128, b: u128,inv: u128, n: u128) -> u128{
   mont_sub_128(mont_prod_128(x,a,inv,n),mont_prod_128(y,b,inv,n),n)
}


const fn gaussian_sqr(x: (u128,u128), inv: u128, n: u128) -> (u128,u128){
    (gaussian_sqr_real(x.0,x.1,inv,n),sqr_coef(x.0,x.1,inv,n))   
}

const fn gaussian_prod(x: (u128,u128), a:(u128,u128),inv: u128, n: u128) -> (u128,u128){
   (gaussian_prod_real(x.0,x.1,a.0,a.1,inv,n),prod_coef(x.0,x.1,a.0,a.1,inv,n))
}

const fn gaussian_pow(mut base: (u128,u128),mut one: (u128,u128),mut pow: u128, inv: u128, n: u128) -> (u128,u128){
   while pow > 1 {
     if pow&1 == 0{
     base = gaussian_sqr(base,inv,n);
     pow>>=1;
   }
    else{
      one = gaussian_prod(one,base,inv,n);
      base = gaussian_sqr(base,inv,n);
      pow>>=1;
    }
  }
  gaussian_prod(one,base,inv,n)
}

// Quadratic integer a+bi where i = sqrt(2)

// x^2 + 2y^2
const fn two_sqr_real(x: u128, y: u128, inv: u128, n: u128) -> u128{
   mont_add(mont_sqr_128(x,inv,n),double(mont_sqr_128(y,inv,n),n),n)
}

const fn two_prod_real(x: u128, y: u128, a: u128, b: u128,inv: u128, n: u128) -> u128{
      mont_add(mont_prod_128(x,a,inv,n),double(mont_prod_128(y,b,inv,n),n),n)
}

const fn two_sqr(x: (u128,u128), inv: u128, n: u128) -> (u128,u128){
    (two_sqr_real(x.0,x.1,inv,n),sqr_coef(x.0,x.1,inv,n))
}

const fn two_prod(x: (u128,u128), a: (u128,u128), inv: u128, n: u128) -> (u128,u128){
   (two_prod_real(x.0,x.1,a.0,a.1,inv,n),prod_coef(x.0,x.1,a.0,a.1,inv,n))
}

const fn two_pow(mut base: (u128,u128),mut one: (u128,u128),mut pow: u128, inv: u128, n: u128) -> (u128,u128){
   while pow > 1 {
     if pow&1 == 0{
     base = two_sqr(base,inv,n);
     pow>>=1;
   }
    else{
      one = two_prod(one,base,inv,n);
      base = two_sqr(base,inv,n);
      pow>>=1;
    }
  }
  two_prod(one,base,inv,n)
}



// x^2 + cy^2
const fn general_sqr_real(x: u128, y: u128, c: u128,inv: u128, n: u128) -> u128{
   mont_add(mont_sqr_128(x,inv,n),mont_prod_128( mont_sqr_128(y,inv,n),c,inv,n),n)
}


const fn general_prod_real(x: u128, y: u128, a: u128, b: u128,c: u128,inv: u128, n: u128) -> u128{
      mont_add(mont_prod_128(x,a,inv,n),mont_prod_128(mont_prod_128(y,b,inv,n),c,inv,n),n)
}

const fn general_sqr(x: (u128,u128),c: u128, inv: u128, n: u128) -> (u128,u128){
   (general_sqr_real(x.0,x.1,c,inv,n),sqr_coef(x.0,x.1,inv,n))
}

const fn general_prod(x: (u128,u128),a: (u128,u128), c: u128, inv: u128, n: u128) -> (u128,u128){
   (general_prod_real(x.0,x.1,a.0,a.1,c,inv,n),prod_coef(x.0,x.1,a.0,a.1,inv,n))
}

const fn general_pow(mut base: (u128,u128),mut one: (u128,u128),c: u128, mut pow: u128, inv: u128, n: u128) -> (u128,u128){
   while pow > 1 {
     if pow&1 == 0{
     base = general_sqr(base,c,inv,n);
     pow>>=1;
   }
    else{
      one = general_prod(one,base,c,inv,n);
      base = general_sqr(base,c,inv,n);
      pow>>=1;
    }
  }
  general_prod(one,base,c,inv,n)
}

// Assuming the GRH we expect to find a nonquadratic residue under 15743 (Ankeny's theorem)
// In: n \in 2Z+1 & n < 2^128
// Out: The minimum element x in the sequence [-1,2,3,4,5,6,...,n] where Jacobi(x,n) =-1
const fn frobenius_idx(n: u128) -> i32{
   // case n = 3 mod 4, half of all odd inputs (n is always odd)
   if n&3 == 3{
      return -1;
   }
   // n = 5 mod 8 or 
   if n&7 == 5{
      return 2;
   }
   
   if n%12 == 5 || n%12 == 7{
      return 3;
   }
   
   if n%5 == 2 || n%5 == 3{
      return 5;
   }
   
   let mut idx = 7;
   // The Frobenius is index is always prime if it is greater than -1   
   while !nqr_128(idx,n){
     idx+=2;
   }
   idx as i32
}

// 1,2,-1, N^-1 will have already been computed for the strong fermat test
pub const fn qft(n: u128, one: u128,two: u128, oneinv: u128,inv: u128) -> bool{
   let idx = frobenius_idx(n);
   let mul_ident = (one,0); 
   // The majority of primes (70%) fall under the first and second cases
   // look at the residue classes in frobenius_idx to see why
   // base-2 strong pseudoprimes seem to map to the first 2 case only 50% of the time
   match idx{
     -1 => {
       // Faster Gaussian case
       
       // initialise starting values
       // 2+1*sqrt(-1)
       let base = (two,one);
       
       let residue = gaussian_pow(base,mul_ident,n,inv,n);
       // Compare to conjugate 2 -1*sqrt(2)
       if residue.0==two && residue.1==oneinv{
          return true;
       }
       false
       }
     2 => {
     // 2+1sqrt(2)
     let base = (two,one);
     let residue = two_pow(base, mul_ident,n,inv,n);
     // Compare to conjugate 2 -1*sqrt(2)
      if residue.0==two && residue.1==oneinv{
          return true;
       }
       false
     }
     _=> {
     
       let base = (one,one);
       let c = to_mont_128(idx as u128,n);
       let residue = general_pow(base,mul_ident, c,n,inv,n);
       // 1 - 1*sqrt(c)
        if residue.0==one && residue.1==oneinv{
          return true;
       }
      false
     }
   }
}
