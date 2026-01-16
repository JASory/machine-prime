// lang_items triggers a warning as of 1.74

//! Machine-prime provides fast implementations of the current best primality tests for 64-bit and optionally 128-bit integers.
//!
//! Machine-prime has 3 different variants. Lucas, SSMR and Tiny. Implementation specifics is given below for each. First the algorithm
//! is described then a general estimate of time complexity is provided. is_prime_wc never uses trial division. 
//! Time complexity is expressed as a ratio of 1 fermat test base-15 and taken as an average of the input candidates. 
//! is_prime's time complexity is approximated from the computation time for the interval [2^64-10^8;2^64]. 
//! is_prime_wc's time complexity is calculated from evaluating 2^64-59.  
//!
//! Additionally there are the Wide and QFT features which extend the functions to 2^128. They are much slower than the previous
//! algorithms due to extended precision arithmetic. They have also not been proven to have no errors below 2^128.
//! 
//! Enabling the Internal feature, exposes the internal arithmetic for integration and reducing code duplication in 
//! other number theory software.  
//! # Default/SSMR
//! Algorithm 
//! - Trial Division by first 129 primes
//! - Base-2 strong fermat test
//! - Look-up table of 262144 candidate bases for a strong fermat test
//! - Branches for n < 2^47 to use a single strong fermat test
//!
//! Properties
//! - is_prime complexity: n < 2^47 0.154; n > 2^47 0.167
//! - is_prime_wc complexity: n < 2^47 1; n > 2^47 2.0
//! - Data Memory: 525472 bytes
//! # Lucas
//! Algorithm 
//! - Trial Division by first 129 primes
//! - Base-2 strong fermat test
//! - Lucas sequence test using a look-up table of parameters
//!
//! Properties
//! - is_prime complexity:  0.2
//! - is_prime_wc complexity: 2.5
//! - Data Memory: 1056 bytes
//! # Tiny/No feature
//! Algorithm 
//! - Divison by 2
//! - Base-2 strong fermat test
//! - Lucas sequence test using parameters calculated over 2Z+1
//!
//! Properties
//! - is_prime complexity: 0.6
//! - is_prime_wc complexity: 2.5
//! - Data Memory: Negligible - no-std Binary compiles to 9.6 kb
//!
//! # Wide
//! Algorithm
//! - Division by first 129 primes (if Lucas or SSMR feature is enabled)
//! - Base-2 strong test
//! - Lucas sequence test
//! 
//! Properties
//! - is_prime_128 complexity: 3.4*t
//! - is_prime_wc_128 complexity: 3.4*t
//!
//! Complexity here is measured against 64-bit. Currently 128-bit runs in approximately 3.4t where t is 64-bit run-time
//! using the "SSMR" algorithm. 
//! Branches to whatever algorithm is selected by other features for n < 2^64. 
//! 
//! This current implementation uses a modified BPSW test which has no known counterexamples. If one wants to strength it, 
//! one can use the strong_fermat test exposed by the "internal" feature, to add more tests at some extra cost. Conversely using
//! QFT can be more efficient.
//!
//! # QFT
//! Algorithm
//! - Division by first 139 primes (if Lucas or SSMR feature is enabled)
//! - Base-2 strong test
//! - Khashin's Quadratic Frobenius test
//! 
//! Properties
//! - is_prime_128 complexity: 4.4*t or 1.3 times the Wide algorithm
//! - is_prime_wc_128 complexity: 9.8*t or 2.8 times the Wide algorithm
//!
//! This algorithm is substantially stronger than the BPSW, and exists as a fallback algorithm incase a BPSW is discovered or the user 
//! wants greater confidence in accuracy. It however has not been proven to be correct for all inputs and errors may exist.


#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]


pub(crate) mod check;
pub(crate) mod hashbase;
pub(crate) mod primes;

#[cfg(any(feature="wide",feature="qft"))]
pub(crate) mod wide;
#[cfg(feature="qft")]
pub(crate) mod qft;

pub use check::{is_prime,is_prime_wc};
#[cfg(any(feature="wide",feature="qft"))]
pub use wide::{is_prime_128,is_prime_wc_128};

#[cfg(feature="internal")]
pub use check::*;
#[cfg(all(feature="internal",feature="wide"))]
pub use wide::*;
#[cfg(all(feature="internal",any(feature="lucas",feature="ssmr")))]
pub use primes::*;
#[cfg(all(feature="internal",feature="ssmr"))]
pub use hashbase::FERMAT_WITNESS;

 // Comment out for crates publication

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/*   End comment out*/
