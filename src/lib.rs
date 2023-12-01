// No std needs to be commented out for stable rustc to compile it

/* Comment out for crates publication */

#![no_std]
#![feature(lang_items)]
#![allow(internal_features)]

/* end publication comment out*/

pub(crate) mod check;
pub(crate) mod hashbase;
pub(crate) mod primes;

pub use check::is_prime;
pub use check::is_prime_wc;

/*  Comment out for crates publication */

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/* End comment out*/
