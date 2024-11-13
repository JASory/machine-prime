Machine prime is a simple, efficient primality test for 64-bit integers, targeting research applications and casual use. 


Two functions are provided with a C-style api to enable calling in other languages. Both can be called in const contexts.

- is_prime
- is_prime_wc (worst case)

is_prime is optimised for the average case and is intended as a general primality test. is_prime_wc is optimised for
the worst case and is intended to be used in functions that the number is already suspected to be prime.It performs
absolutely minimal checks, and is permitted to have a handful of known failure points leaving it up to the user to
decide if it is necessary to expend the cost to check for them.

This function api will never change. is_prime will always correctly evaluate any 64-bit integer, however is_prime_wc
failure points may change.See source code or documentation for a list of these failure points. 

Four modes are available for these functions, Default, SSMR and Small and Tiny. Memory use decreases but the complexity
increases with each successive mode. 

SSMR is the fastest for small values (n< 2^40) but may be slower than Default for larger values (n > 2^40). This is 
the recommended version to be used, since most applications are going to be smaller primes, and is_prime_wc is nearly
twice as fast for (n < 2^40).

## Usage
 Due to some barriers to compiling no-std libraries, the version in crates.io and the repository are slightly different,
 although using the exact same algorithms. crates.io version is configured for stable compilers, while the repository 
 is configured for nightly compiler, as the intention is to use it to build dynamic libraries. 
 
 To use from crates.io, simply include it in your cargo.toml file with the feature "ssmr","small" or "tiny" if you want those versions. 
 
 To use as a dynamic library, make sure you are running rustc nightly; `git clone` this repository and run `make`
 to compile the Default mode, 'make ssmr' for the SSMR mode, `make small` for the Small mode and `make tiny` for
 the Tiny mode. This will create the library, and `make install` will install it to `/lib/libprime.so`. 
 (Make sure you are superuser). Installing the library is recommended but not strictly necessary. Link to it using 
 ``-lprime`` if you are using gcc. 

Alternately, if on Windows, use the provided batch file. 

See the ["binding"](https://github.com/JASory/machine-prime/tree/main/binding) folder in the repository for 
various language bindings. Several languages including the following are supported. 
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
  
##  Notes
See [Performance](https://github.com/JASory/machine-prime/blob/main/PERFORMANCE.md) for some benchmarks and 
comparison to other efficient primality tests. 

Approximate size of binary dynamic library varies with your compiler however the following are reasonable estimates:
- Default/SSMR-542.4kb
- Small-18.1kb
- Tiny-14kb

Building for embedded systems has not been tested, however there is no known cause for failure.

This software is in public domain, and can be constructed in a reproducible manner with the 
[f-analysis](https://github.com/JASory/f-analysis) library, and Feitsma/Galway's base-2 
pseudoprime table. It is [available on crates.io](https://crates.io/crates/machine-prime)
