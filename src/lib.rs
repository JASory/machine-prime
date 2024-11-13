

// No std needs to be commented out for stable rustc to compile it
// lang_items triggers a warning as of 1.74

//! Machine-prime has 4 different variants. Default, SSMR, Small, and Tiny. Implementation specifics is given below for each. First the algorithm
//! is described then a general estimate of time complexity is provided. is_prime_wc never uses trial division. 
//! Time complexity is expressed as a ratio of 1 fermat test base-15 and taken as an average of the input candidates. 
//! is_prime's time complexity is approximated from the computation time for the interval [2^64-10^8;2^64]. 
//! is_prime_wc's time complexity is calculated from evaluating 2^64-59.  
//!
//! # Default
//! Algorithm 
//! - Trial Division by first 67 primes
//! - Euler-Plumb primality test
//! - Look-up table of 262144 candidate bases for a strong fermat test
//!
//! Properties
//! - is_prime complexity: 0.163
//! - is_prime_wc complexity: 2.0
//! - Data Memory : 525072 bytes
//! # SSMR
//! Algorithm 
//! - Trial Division by first 67 primes
//! - Euler-Plumb primality test
//! - Look-up table of 262144 candidate bases for a strong fermat test
//! - Branches for n < 2^40 to use a single strong fermat test
//!
//! Properties
//! - is_prime complexity: n < 2^40 0.154; n > 2^40 0.167
//! - is_prime_wc complexity: n < 2^40 1; n > 2^40 2.0
//! - Data Memory: 525072 bytes
//! # Small
//! Algorithm 
//! - Trial Division by first 67 primes
//! - Euler-Plumb primality test
//! - Lucas sequence test using a look-up table of parameters
//!
//! Properties
//! - is_prime complexity:  0.2
//! - is_prime_wc complexity: 2.45
//! - Data Memory: 811 bytes
//! # Tiny
//! Algorithm 
//! - Divison by 2
//! - Euler-Plumb primality test
//! - Lucas sequence test using parameters calculated over 2Z+1
//!
//! Properties
//! - is_prime complexity: 0.6
//! - is_prime_wc complexity: 2.45
//! - Data Memory: Negligible - no-std Binary compiles to 9.6 kb


/* Comment out for crates publication
*/
#![no_std]
#![feature(lang_items)]
/*
  end publication comment out */

pub(crate) mod check;
pub(crate) mod hashbase;
pub(crate) mod primes;

pub use check::is_prime;
pub use check::is_prime_wc;
/*
  Comment out for crates publication
*/
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
/*
   End comment out*/
