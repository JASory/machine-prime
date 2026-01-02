use machine_prime::{strong_fermat_128,one_mont_128,two_mont_128,to_mont_128,lucas_128,mul_inv2_128};

/*

  Demonstration of an almost surely correct deterministic primality test for 2^64 < n < 2^128
  
  Argument:  All semiprimes of the form (2x+1)(4x+1) are eliminated by the fermat witnesses. 
  There are probably no Carmichael numbers of the form p,q,r 3 mod 4 considering that they are much rarer than 
  the semiprimes. Likewise we expect no other composites to pass the fermat component, the most probable candidate
  is of the form (4x+1)(12x+1), with a probability of existing less than 0.065. 
  Additionally there are no known pseudoprimes to 2 and the lucas test under 2^64. The few pseudoprimes to both a
  lucas test and a strong fermat test, occur at a rate of approximately 1/42000 per fermat pseudoprime. Therefore
  in the worst case we can estimate a counterexample exists with a probability of less than 1.5E-6.  
  
  
  
  Note that running this example requires using the "internal" feature
  
  A practical implementation will employ trial division like the library function but this is omitted 
  for simplicity.  A slighly simpler implementation would be to remove the base 2 and lucas test and run the test
  after the library is_prime_128 function. The initialisation values would have to be computed twice, however. 
  
  Approximate run time 21.5 Fermat tests
*/

fn strong_test(x: u128) -> bool{
   // We cannot use this algorithm for n < 2^64 due to a Montgomery transform "error" 
   // when using 64-bit inputs for a 128-bit transform.
    debug_assert!(x > 1u128<<64);
    // X must be odd to ensure that the inverse over 2^128 can be calculated
    debug_assert!(x&1==1);
    
   // Initialise necessary values
   
    let inv = mul_inv2_128(x);

    let tzc = x.wrapping_sub(1).trailing_zeros();
    let one = one_mont_128(x);
    let oneinv = x.wrapping_sub(one);
    let two = two_mont_128(one, x);

   // Witness using 2. Nearly all composites will be eliminated by this
   // You can remove this if it is being used as a complement to the is_prime_128 library function
    if !strong_fermat_128(x, tzc, two, one, oneinv, inv) {
        return false;
    }

   // Fermat component alone should be sufficient
   // The witnesses were computed from pseudoprimes to the prime witnesses 2;47
   for i in [3,5,7,11,13,17,19,23,29,31,37,41,43,47,511,659,679,8129,70157]{
       let montbase = to_mont_128(i,x);
       
       if !strong_fermat_128(x,tzc,montbase,one,oneinv,inv){
          return false;
       }
   }
   
   // Add a Lucas sequence test
   // Also removable if used as a complement
    lucas_128(x,one,two,inv)
}


