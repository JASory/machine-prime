#![no_std]

pub(crate) mod check;
pub(crate) mod hashbase;
pub(crate) mod primes;
pub use check::is_prime_ac;
pub use check::is_prime_wc;


// This function is called on panic.
#[cfg(no_std)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
