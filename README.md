Machine prime is a simple efficient primality test for 64-bit integers, 
derived from algorithms used in the [number-theory](https://crates.io/crates/number-theory) crate

Two functions are provided with a C-style api to enable calling in other languages

is_prime_wc and is_prime_ac

is_prime_wc (worst case) performs minimal branching, zero trial division and returns incorrect results for 
1, and powers of 2
It is intended as a fast check of numbers that are already likely to be prime (aka the worst case complexity)

Worst-case  2xFermat_test
Average-case 2xFermat_test

is_prime_ac (average case) utilizes trial division and branching to ensure that the results are correct and efficiently evaluated in the average case

Worst-case 2.4xFermat_test
Average-case 0.3xFermat_test


This is fairly close to the optimal in practice. An implementation using less than 2 fermat tests 
would require very large amounts of memory, that would bottleneck performance. 

Total Memory used:  1050752 bytes

Requires 
 x86-64 architecture, preferably with 128-bit arithmetic support for efficiency purposes
 
This library (unlike number-theory) does not implement the most efficient implementation for all integers,
but rather a simpler one that is efficient in the entire interval.This can actually result in faster average
case, due to eliminating branching. number-theory (and an upcoming crate) will be faster for integers less than 2^35. 
However this is only 1/2^29 th of the interval meaning that the branching would be too great a penalty for the majority of calls.  
 
Purpose: 
Many number theoretic functions either require a primality test or use a primality test for greater
efficiency (Legendre symbol, factorization).
While primality testing is rarely the bottleneck for these functions, it is a limiting factor in highly
optimized computations. By providing an fast implementation that can be  called in many languages, the work towards constructing such a function is minimised.

Note: This is tied with Forisek & Jancina's [FJ64_262K.cc](https://people.ksp.sk/~misof/primes/FJ64_262K.cc) implementation for theoretical worst-case complexity, however machine-prime is considerably faster (3x <) in actual implementation, due to 128-bit arithmetic, Hensel lifting and prime-inverses. 
