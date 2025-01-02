// No std needs to be commented out for stable rustc to compile it
// lang_items triggers a warning as of 1.74

//! Machine-prime has 4 different variants. Lucas, Table, SSMR and Tiny. Implementation specifics is given below for each. First the algorithm
//! is described then a general estimate of time complexity is provided. is_prime_wc never uses trial division. 
//! Time complexity is expressed as a ratio of 1 fermat test base-15 and taken as an average of the input candidates. 
//! is_prime's time complexity is approximated from the computation time for the interval [2^64-10^8;2^64]. 
//! is_prime_wc's time complexity is calculated from evaluating 2^64-59.  
//!
//! Additionally there is the Wide feature which extends the functions to 2^128. This is much slower than the previous algorithms and may
//! pass composites.
//! 
//! Enabling the Internal feature, exposes the internal arithmetic for integration and reducing code duplication in 
//! other number theory software.  
//! # Default/Table
//! Algorithm 
//! - Trial Division by first 67 primes
//! - Base-2 strong fermat test
//! - Look-up table of 262144 candidate bases for a strong fermat test
//!
//! Properties
//! - is_prime complexity: 0.163
//! - is_prime_wc complexity: 2.0
//! - Data Memory : 525072 bytes
//! # SSMR
//! Algorithm 
//! - Trial Division by first 67 primes
//! - Base-2 strong fermat test
//! - Look-up table of 262144 candidate bases for a strong fermat test
//! - Branches for n < 2^47 to use a single strong fermat test
//!
//! Properties
//! - is_prime complexity: n < 2^47 0.154; n > 2^47 0.167
//! - is_prime_wc complexity: n < 2^47 1; n > 2^47 2.0
//! - Data Memory: 525072 bytes
//! # Lucas
//! Algorithm 
//! - Trial Division by first 67 primes
//! - Base-2 strong fermat test
//! - Lucas sequence test using a look-up table of parameters
//!
//! Properties
//! - is_prime complexity:  0.2
//! - is_prime_wc complexity: 2.5
//! - Data Memory: 811 bytes
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
//! - Division by first 67 primes
//! - Base-2 strong test
//! - Lucas sequence test
//! 
//! Properties
//! - is_prime_128 complexity: 3.4*t
//! - is_prime_wc_128 complexity: 3.4*t
//! Complexity here is measured against 64-bit. Currently 128-bit runs in approximately 3.4t where t is 64-bit run-time
//! using the "Table" algorithm. 
//! Branches to whatever algorithm is selected by other features for n < 2^64. 
//! 
//! This current implementation uses a modified BPSW test which has no known counterexamples. If one wants to strength it, 
//! one can use the strong_fermat test exposed by the "internal" feature, to add more tests at some extra cost.  

/* Comment out for crates publication
*/
#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]
/*
  end publication comment out */
#![allow(improper_ctypes_definitions)]
pub(crate) mod check;
pub(crate) mod hashbase;
pub(crate) mod primes;

#[cfg(feature="wide")]
pub(crate) mod double;

pub use check::{is_prime,is_prime_wc};
#[cfg(feature="wide")]
pub use double::{is_prime_128,is_prime_wc_128};

#[cfg(feature="internal")]
pub use check::*;
#[cfg(all(feature="internal",feature="wide"))]
pub use double::*;
#[cfg(all(feature="internal",any(feature="lucas",feature="table",feature="ssmr")))]
pub use primes::*;
#[cfg(all(feature="internal",any(feature="table",feature="ssmr")))]
pub use hashbase::FERMAT_TABLE;
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
