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
 Machine-prime has 5 features, 2 of which are exclusive, and 2 of which can be combined with others
 - (None) - Accessible by default-features=false. This simply employs a 64-bit BPSW
 - Lucas - Uses parameter optimisations and adds trial division for much faster average case (exclusive)
 - SSMR - Single-Shot Miller-Rabin; uses a witness table for one witness check for n < 2^47 and two for n > 2^47 (exclusive)
 - Wide - Implements primality for 2^64;2^128. A strong base-2 and Lucas-V test are used.
 - QFT  - Wide variant that replaces the Lucas-V test with Khashin's stronger QFT. Takes precedence over Wide.
 - Internal - Exposes internal algorithms and data, in Rust api (not C-api)
 
 
 Machine-prime implements feature precedence going Lucas -> SSMR, and Wide -> QFT. In other words if SSMR is implemented in one dependency
 it will override another Machine-prime dependency using "Lucas" feature. Likewise, "qft" will override "wide".
 By default, Machine-prime implements the "SSMR" feature. Overriding this with default-features=false will fallback to a slower variant of
 the "Lucas" algorithm. It is strongly recommended that you use one of the features, as this algorithm is quite slow in the average case. 
 
 Implementing "Wide" adds the is_prime_128 and is_prime_wc_128 functions, which branches to whichever algorithm the 
 other features define for values below 2^64, and uses a BPSW test for n > 2^64.
 If you use feature "QFT" this is replaced with a strong fermat test with witness 2 and Khashin's QFT. Essentially a much stronger if substantially
 slower BPSW. Neither test is guaranteed to be correct, but neither have any known errors. Khashin's QFT has been verified up to 2^64, and BPSW
 up to 2^81. 
 
 Trial division is accessed with the features "Lucas" or "SSMR" this will include trial division for 128-bit arithmetic as well if the features
 "wide" or "qft" are used. If you want to avoid using trial division, call the is_prime_wc variants or you can entirely omit the
 trial division data by compiling it with default-features=false, and adding either "wide" or "qft" feature.
 
 Implementing the "internal" feature exposes the internal arithmetic and data used. You cannot call these functions outside of Rust.
 
 ## Bindings
 To use as a dynamic library, make sure you are running rustc nightly; `git clone` this repository and run `make`
 to compile the default algorithm, `make lucas` for the "Lucas" mode, and `make ssmr` for
 the SSMR mode. To extend to 128-bit add "wide" or "qft" to the feature word, e.g `make ssmrwide`. 
 Calling 128-bit FFI may result in errors. 
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
  - Searching for "rare" forms of primes, used to construct Carmichael numbers, Monier-Rabin semiprimes and other special composites. An example of this is searching for strong pseudoprimes of form (2x+1)(4x+1) to the first 15 primes; less than 1/50000 of primes are eligible candidates.
  
is_prime_wc is much more useful as it is faster against numbers that are already very likely to be prime

  - Factorisation; determining that the factors returned are actually primes, and when to halt the factorisation
  - Legendre symbol
  - Verifying Erastothenes sieves, and other primality tests; checking that prime_test(n) == is_prime_wc(n)
  - Verifying small primes used in prime certification, like the Pratt certificate
  - Searching for Fermat pseudoprimes; verifying that certain numbers that satisfy a^(n-1) mod n =1 are infact composite
 
##  Notes

See [Performance](https://github.com/JASory/machine-prime/blob/main/PERFORMANCE.md) for some benchmarks and 
comparison to other efficient primality tests. 

Approximate size of binary dynamic library varies with your compiler however the following are reasonable estimates after stripping debug symbols:
- SSMR-QFT - 548kb
- Default/SSMR-532kb
- Lucas-9.5kb
- Tiny-7.8kb

Building for embedded systems has not been tested, however there is no known cause for failure.

This software is in public domain, and can be constructed in a reproducible manner with the 
[f-analysis](https://github.com/JASory/f-analysis) library, and Feitsma/Galway's base-2 
pseudoprime table. It is [available on crates.io](https://crates.io/crates/machine-prime)

## References 
 QFT Algorithm - Sergei Khashin. [Evaluation of the Effectiveness of the Frobenius Primality Test](https://arxiv.org/pdf/1807.07249).2020
 
 Montgomery arithmetic routines - Ernest Mayer. [Efficient long division via Montgomery multiply](https://arxiv.org/abs/1303.0328). 2016.
 
 Lucas-V and Strong fermat test - Crandall, Pomerance. Primes: A Computational Perspective.2000
 
 Witness table adapted from the algorithm in this paper - Viorel Wegner. [The Single Shot Miller-Rabin Test](https://www.researchgate.net/publication/396531974_The_Single-Shot_Miller_Rabin_Test?_tp=eyJjb250ZXh0Ijp7ImZpcnN0UGFnZSI6ImxvZ2luIiwicGFnZSI6InNlYXJjaCIsInBvc2l0aW9uIjoicGFnZUhlYWRlciJ9fQ). 2025
 
 
 With substantial contribution by David Sparks.
