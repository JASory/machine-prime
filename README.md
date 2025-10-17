Machine prime is a simple, efficient primality test for 64-bit integers, targeting research applications and casual use. 
Enabling the "wide" feature allows primality testing for 128-bit integers.

Two functions are provided with a C-style api to enable calling in other languages. Both can be called in const contexts.

- is_prime
- is_prime_wc (worst case)

is_prime is optimised for the average case and is intended as a general primality test. is_prime_wc is optimised for
the worst case and is intended to be used in functions that the number is already suspected to be prime.It performs
absolutely minimal checks, and is permitted to have a handful of known failure points leaving it up to the user to
decide if it is necessary to expend the cost to check for them.

This function api will never change. is_prime will always correctly evaluate any 64/128-bit integer, however is_prime_wc
failure points may change.See source code or documentation for a list of these failure points.

## Usage
 Machine-prime has 5 features, 3 of which are exclusive, and 2 of which can be combined with others
 
 - Lucas - Uses a lucas-v test and  adds trial division
 - Table - Uses a hashtable 
 - SSMR - branches for faster primality for n < 2^47 
 - Wide - implements primality for 2^64;2^128
 - Internal - Exposes internal algorithms and data, in Rust api (not C-api)
 
 
 Machine-prime implements feature precedence going Lucas -> Table -> SSMR. In other words if the Table is implemented in one dependency
 it will override another Machine-prime dependency using "Lucas" feature. Likewise SSMR will override both "Table" and "Lucas".
 By default, Machine-prime implements the "Table" feature. Overriding this will implement a slower variant of the "Lucas" algorithm. It is
 strongly recommended that you use one of the features, as this algorithm is very slow in the average case. 
 
 Implementing "Wide" redefines the function to accept 128-bit, which branches to which algorithm the other features define for values below
 2^64, and uses a BPSW test for n > 2^64.
 Implementing the "internal" feature exposes the internal arithmetic and data used. You cannot call these functions outside of Rust.  
 
 ## Bindings
 To use as a dynamic library, make sure you are running rustc nightly; `git clone` this repository and run `make`
 to compile the default algorithm, `make lucas` for the "Lucas" mode, `make table` for the Table mode and `make ssmr` for
 the SSMR mode. To extend to 128-bit add "wide" to the feature word, e.g `make tablewide`. Calling 128-bit FFI may result 
 in errors. 
 `make install` will install the created library to `/lib/libprime.so`.(Make sure you are superuser). 
 Installing the library is recommended but not strictly necessary. Link to it using 
 ``-lprime`` if you are using gcc. 

Alternately, if on Windows, use the provided batch file. 

See the ["binding"](https://github.com/JASory/machine-prime/tree/main/binding) folder in the repository for 
various language bindings. Several languages including the following are supported. 128-bit may not work on all architectures. 
- Ada
- C/C++ 
- Fortran 
- Haskell
- Julia 
- Python

## Applications
  Due to Machine-primes speed, quite a few applications can be found for it, including in research projects.
  
  is_prime
  - Searching for primes, particularly in intervals that would require large computations for sieves, or primes out of sequence. 
  - Searching for "rare" forms of primes, used to construct Carmichael numbers, Monier-Rabin semiprimes and other special composites.
  
is_prime_wc is much more useful as it is faster against numbers that are already very likely to be prime

  - Factorisation; determining that the factors returned are actually primes, and when to halt the factorisation
  - Legendre symbol
  - Verifying Erastothenes sieves, and other primality tests; checking that prime_test(n) == is_prime_wc(n)
  - Verifying small primes used in prime certification, like the Pratt certificate
  - Searching for Fermat pseudoprimes; verifying that certain numbers that satisfy a^(n-1) mod n =1 are infact composite

Machine-prime is wrapped in a more convenient library [c-prime](https://crates.io/crates/c-prime) and an optional dependency speed up in 
Sorngard's [const-primes](https://crates.io/crates/const-primes) library. 
 
##  Notes
See [Performance](https://github.com/JASory/machine-prime/blob/main/PERFORMANCE.md) for some benchmarks and 
comparison to other efficient primality tests. 

Approximate size of binary dynamic library varies with your compiler however the following are reasonable estimates after stripping:
- SSMR-Wide - 540kb
- Default/SSMR-533kb
- Lucas-10kb
- Tiny-7kb

Building for embedded systems has not been tested, however there is no known cause for failure.

This software is in public domain, and can be constructed in a reproducible manner with the 
[f-analysis](https://github.com/JASory/f-analysis) library, and Feitsma/Galway's base-2 
pseudoprime table. It is [available on crates.io](https://crates.io/crates/machine-prime)

Citation for the algorithm used to calculate the hashtable

Viorel Wegner. _The Single-Shot Miller Rabin Test_. (2025). [ResearchGate](https://www.researchgate.net/publication/396531974_The_Single-Shot_Miller_Rabin_Test?_tp=eyJjb250ZXh0Ijp7ImZpcnN0UGFnZSI6ImxvZ2luIiwicGFnZSI6InNlYXJjaCIsInBvc2l0aW9uIjoicGFnZUhlYWRlciJ9fQ)
