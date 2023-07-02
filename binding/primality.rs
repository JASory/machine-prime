
// rustc file.rs -lprime

#[link(name = "prime")]
extern {
   fn is_prime(x: u64) -> bool;
   
   fn is_prime_wc(x: u64) -> bool;
}

/*
                Benchmark

   This benchmark is identical to the machine-prime crates "speed" benchmark
   
   C and Fortran bindings has approximately the same execution speed 5s and 6s respectively
   
 
            Native Rust      Rust Binding
 Top 10^8    5.08s                  4.86s
 Strongest   6.379s                 6.0s

*/

