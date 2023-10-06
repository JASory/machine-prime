Performance 

Machine-prime is compared to some of the other fastest primality tests for 64-bit integers. 

The benchmarking is performed by starting from the 2^n and counting down 100 million integers. The values tested are from 2^31 to 2^64. By iterating sequentially one avoids overhead in generating random numbers, and one can expect the multiples of primes (composites) to be roughly evenly distributed or sufficiently that it would not skew the results by the degree of difference that we observe. 


## Algorithms compared
- FJ262K Forisek and Jancina's 262,144 parameter 2-base test, despite the suboptimal implementation it was apparently the fastest stand-alone primality test, and possibly the fastest period before number-theory. 
- num-prime Zhong's implementation of Forisek and Jancina's 16384 parameter 3-base test
- number-theory Sory's hybrid algorithm using 16384 parameter 2-base test with a special semiprime check 
- Mp Default algorithm using Colin Plumb's euler test with a 262,144 parameter strong fermat test (computationally equivalent to 1.95 bases)
- Mp (Small) Euler-Plumb with simplified Lucas V (computationally equivalent to 2.45 bases) 
- Mp (Worst Case) Identical to Mp Small, except calling is_prime_wc. This performs zero trial division (the only test here that omits it) making it the worst possible test for checking random integers. It is however faster against primes, this benchmark is meant to show that slowest possible implementation. 


All other publicly available primality tests are believed to be considerably slower, often by orders of magnitude. (Note Bradley Berg's algorithm would in theory be comparable to other algorithms here, however as noted elsewhere by Sory, the algorithm parameters are incorrect and only correct implementations are in consideration).

## Result

![Primality benchmark](https://github.com/JASory/machine-prime/blob/main/primality.png)

The fastest primality tests against random composites in order are 

- Machine-prime Default
- Machine-prime Small
- Number-theory 0.0.23
- num-prime 0.3.2
- Machine-prime Worst Case
- FJ262K


Keep in mind that half of all 64-bit integers are greater than 2^63, so the average case is dominated by the right-hand side of the graph. For instance Mp Worst Case is faster than FJ262K for all integers greater than 2^62, this is 75% of all integers more than sufficient to outweigh the few smaller intervals that FJ262K is faster. Another instance is number-theory is the fastest for integers less than 2^35, however this is actually only 1/500 million of the permitted input. 

Additionally we benchmark against the strongest case (2^64-59) evaluated 10 million times. Not surprisingly Machine-prime also dominates this benchmark, not the least of which is the is_prime_wc function which removes all checks except the minimum to prove primality. 

- Machine-prime Default 8.05  is_prime_wc 7.54
- Machine-prime Worst Case 9.06
- Machine-prime Small 9.98, is_prime_wc 9.59
- number-theory 13.8933
- num-prime 15.64
- FJ262K 27.6

Machine-primes primary advantages varies depending on which algorithm it is compared to, however three apparently unique optimisations are the usage of Plumbs euler fermat variant, the use of prime-inverse multiplication instead of division, and 
removal of repeated computation, as well as a faster hash-function in the Default algorithm. In theory FJ262K should be the second-fastest however it's reliance on naive trial division, and using a gcd function to compute inverses instead of hensel-lifting, results in poor actual performance. num-prime has a decent implementation, however it suffers from poor trial division (albeit not entirely naive) and a suboptimal 3-base check and hash function. number-theory has a good implementation, however it can be improved by usage of a faster euler-plumb check, and shared computation these are weaknesses of the other algorithms as well. An additional concern is the unique reliance on floating-point arithmetic.Note that the version numbers here are important, number-theory will likely use Machine-prime in the future. 
Machine-prime Default (i.e the algorithm used if no features are selected) is the largest in memory consumption and the fastest. Machine-prime (Small) is the second smallest using only 67 integers for division and 256 bytes to compute inverses, as well as the second fastest. 
Machine-prime worst case  removes these  as well as checks for even integers resulting in very little memory consumption however it is the second-slowest. Calling is_prime in the Mp (Tiny) configuration is about 2x as fast however  it is still the second slowest due to the wide gap between FJ262K and num-prime. 
number-theory 0.0.23 is slightly faster for integers less than 2^35 because it uniquely uses only a single fermat test for them, however this is such a small amount as to be inconsequential in the average case. 

So the final result is that the Machine-prime variants are the fastest or most memory-efficient if the correct feature is selected for. 
