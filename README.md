Machine prime is a simple efficient primality test for 64-bit integers, 
constructed in a reproducible manner with the [f-analysis](https://github.com/JASory/f-analysis) library, 
and Feitsma/Galway's base-2 pseudoprime table. (Note that while f-analysis does use this library, it is not a circular dependency as it is not actually used in the computations to produce the fermat bases or hashtable. It is primarily to strip primes from a set of integers before evaluation, and some heuristic pseudoprime generation)

Two functions are provided with a C-style api to enable calling in other languages.

- is_prime
- is_prime_wc (worst case)

is_prime is optimised for the average case and is intended as a general primality test. is_prime_wc is optimised for the worst case and is intended to be used in functions that the number is already suspected to be prime. for instance factorisation where trial division has already been performed. It performs absolutely minimal checks, and is permitted to have a handful of known failure points leaving it up to the user to decide if it is necessary to expend the cost to check for them.

This function api will never change. is_prime will always correctly evaluate any 64-bit integer, however is_prime_wc  failure points may change.

Three modes are available for these functions, Default and Small and Tiny. Memory use decreases but the complexity increases with each successive mode. 

The Default utilizes a large hashtable, and trial division by prime inverse multiplication

 - is_prime complexity: Worst-case 2.23 * Fermat test, Average-case 0.3 * Fermat test
 - is-prime_wc complexity: Worst-case 2 * Fermat test, Average-case 1.2* fermat test
 - is_prime_wc failures : perfect powers of 2, 1, panics at 0
 - Hashtable constructed by f-analysis' `to_hashtable(Some(262144),Some(1069587295),Some(65535))`. Compute the base-2 strong pseudoprimes to Feitsma's table 
   and apply the hashtable method to reproduce the hashtable used. See the "hashtable" example in f-analysis for an explicit implementation. 
 - Total memory: 1050752 bytes
 
Small forgoes the hashtable but still uses the trial division
  - is_prime complexity: Worst-case 8.23 * Fermat test, Average case 0.567 * Fermat test
  - is-prime_wc complexity: Worst case 8 * Fermat test, Average-case 1.31 * Fermat test
  - is_prime_wc failures: Perfect powers of 2,1, panics at 0 
  - Fermat bases constructed using f-analysis `iter_sprp_search(80)`, on the set of base-2 strong pseudoprimes
  - Fermat bases used: 2,60,52,37,79,29,41,55

Tiny simply uses the Fermat bases implemented in Small, the only difference therefore between is_prime and is_prime_wc is the latter forgoes any additional checks to ensure correctness. It saves a small amount of memory over Small. 


## Usage
 Due to some barriers to compiling no-std libraries, the version in crates.io and the repository are slightly different, although using the exact same algorithms. 
 crates.io version is configured for stable compilers, while the repository is configured for nightly compiler, as the intention is to use it to build dynamic libraries. 
 
 To use from crates.io, simply include it in your cargo.toml file with the feature "small" or "tiny" if you want those versions. Default will be the fastest with the hashtable.
 
 To use as a dynamic library, make sure you are running rustc nightly; `git clone` this repository and run `make` to compile the Default mode, `make small` for the Small mode and `make tiny` for the Tiny mode. This will create the library, and `make install` will install it to `/usr/lib/libprime.so`. (Make sure you are superuser). Installing the library is recommended but not strictly necessary. Link to it using ``-lprime`` if you are using gcc. 

## Purpose
Many number-theoretic functions either require a primality test or use a primality test for greater efficiency. Examples include the Legendre symbol and factorization.While primality testing is rarely the bottleneck for these functions, it is a limiting factor in highly
optimized computations. By providing an fast public domain implementation that can be  called in many languages and architectures, 
the work towards constructing such a function is minimised.

## Notes
This primality test is not intended to be the fastest in every interval, but rather faster in general and the average case. There are small tests that are faster for 32-bit, however this is such a miniscule interval that branching to account for them would result in a reduction in efficiency in general. 

There are fast functions that can be used to optimise  for memory better. However, they use floating point arithmetic. The intent of this library is to be extremely fast, but flexible enough to be used wherever integer arithmetic is supported. This is why low-memory variants are also supported.Not every user can , or wants to, use nearly a mebibyte of memory.

The Small and Tiny modes use 8 fermat bases instead of the minimum 7 bases because the 7-base set has large factors, making it more difficult to correct for. 

Future variants will likely use a Lucas pseudoprime test for the low-memory modes. Euler_Plumb may be investigated as a replacement for the 2-sprp check. 

The Default mode is currently tied with Forisek & Jancina's [FJ64_262K.cc](https://people.ksp.sk/~misof/primes/FJ64_262K.cc) implementation for theoretical worst-case complexity, however machine-prime is considerably faster (3x <) in actual implementation, due to 128-bit arithmetic, Hensel lifting and prime-inverses.

This software is in public domain, and all the values can be deterministically recomputed using the open-source auditable f-analysis library.
