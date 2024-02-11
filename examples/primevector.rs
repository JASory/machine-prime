use machine_prime::is_prime;

/*
   Some reference implementations
   for prime vectors and iterators, primarily convenience functions
   
   PrimeVector is novel implementation of attempting to speed up primality checking of
   
   PrimeIter is a trivial iterator over the set of 64-bit primes
   
   ResiduePrime is a realistic number-theoretic application that shows the real purpose of machine-prime;
   checking primality of numbers of some specific form  across the entire interval (0;2^64], 
   without costly initialisation of all primes

*/

/*

  The worst case of primality testing is against primes
  Fortunately if we can expect to be testing some unknown
  but small set of primes frequently, we can store them
  as we encounter them and perform a faster lookup
  
  Unfortunately Machine-prime is fast enough that this optimisation 
  only works around less than 150K primes. Slower checks like used in primal crate
  will benefit more from this optimisation.
*/

struct PrimeVector{
  primes: std::collections::HashSet::<u64>,
}

impl PrimeVector{

 fn new()  -> Self{
    Self{primes: std::collections::HashSet::<u64>::new()}
 }

// Check if a number is prime, if true then store it for faster lookup later
  fn check(&mut self, x: u64) -> bool{

     if self.primes.contains(&x){
       return true;
     }
     else{
         if is_prime(x){
          self.primes.insert(x);
             return true;
         }
        return false
       }
     }
}

// Prime iterator, normally this would be constructed with an Erastothenes sieve
// While Machine-prime is generally slower than sieves, it still generates about 300k primes per second
#[derive(Copy,Clone)]
struct PrimeIter{
  p: u64
}

impl PrimeIter{
  fn new(p: u64) -> Self{
     PrimeIter{p}
  }
  // Sugar to make it look like a normal iterator
  fn iter(&self) -> Self{
     self.clone()
  }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {

      loop{
        if self.p > u64::MAX-57{
           return None
        }
        self.p += 1;
          if is_prime(self.p){
            return Some(self.p)
          }
        }
    }
    
   fn size_hint(&self) -> (usize,Option<usize>){
       (2usize,Some((u64::MAX-58) as usize))
   } 
    
   fn last(self) -> Option<Self::Item>{
      Some(u64::MAX-58)
   }
}

impl DoubleEndedIterator for PrimeIter{
      
    fn next_back(&mut self) -> Option<Self::Item>{

      loop{
        if self.p < 1{
           return None
        }
        self.p -= 1;
          if is_prime(self.p){
            return Some(self.p)
          }
        }
    }
}


// Primes of  the form x*Ring + A  i.e  x \equiv A (mod Ring)
#[derive(Copy,Clone)]
struct ResiduePrime<const A: u64,const RING: u64 >{
  p: u64
}


impl<const A: u64, const RING: u64> ResiduePrime<A,RING>{


  fn new(p: u64) -> Option<Self>{
   // Checks that starting value is already a valid residue,
   // but not necessarily prime
    if p%RING == A{
      return Some(Self{p})
    }
    None
  }
  
}

impl <const A: u64, const RING: u64> Iterator for ResiduePrime<A,RING>{
     type Item = u64;
     
   fn next(&mut self) -> Option<Self::Item>{
     
      loop{
        if self.p > u64::MAX-57{
           return None
        }
        self.p += RING;
        let x : u64 = self.p;
          if is_prime(x){
            return Some(x)
          }
        }
   }  
}


fn main(){
  let mut prime_checker = PrimeVector::new();
  const BOUND: u64 = 6_000_000;
  const INF : u64 = 1<<60;
  const SUP : u64 = INF + BOUND;
  let mut count = 0u64;
  
  let start = std::time::Instant::now();
   
  for i in INF..SUP{
     if is_prime(i){
       count+=1;
     }
  }
   
  let stop = start.elapsed();
  println!("{} primes counted in {:?} using plain Machine-prime",count,stop);
  
  // Initialise 
  for i in INF..SUP{
     prime_checker.check(i);
  }
  
   count=0u64;
   
  let start = std::time::Instant::now();
   
  for i in INF..SUP{
     if prime_checker.check(i){
       count+=1;
     }
  }
   
  let stop = start.elapsed();
   
    println!("{} primes counted in {:?} using a hashset, and machine-prime",count,stop);
   
  let mut count = 0u64;
   
  let P = PrimeIter::new(2);
    let start = std::time::Instant::now();
   
  for primes in P.iter(){
   count+=1;
    if count > 1_000_000{
       break;
    }
  }
   
    let stop = start.elapsed();
  println!("Sequentially generated 1 million primes in t: {:?}",stop);
  
  // Some candidate factors for base 15 fermat pseudoprimes
  let candidate_15 = ResiduePrime::<1,29130>::new(1).unwrap();
   
      let start = std::time::Instant::now();
      let mut count = 0u64;
   
  for i in candidate_15{
    count+=1;
   if i > 1u64<<32{
      break;
   }
  }
   
  let stop = start.elapsed();
   
  println!("Found all {} fermat pseudoprime candidates to base-15 of the form 29131n where n is prime under {} in t: {:?}",count,29131*(1u64<<32),stop);
   
}
