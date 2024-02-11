These are bindings to machine-prime for various languages. Most languages that support generic integers have generic implementations 
written for them, keep in mind that these are all direct calls so signed integers or 128-bit or larger integers will be passed directly
and return erroneous values.

Each file is for a different language. They contain instructions on how to compile and link to the machine-prime library. 
These all assume that it was compiled and saved as `/lib/libprime.so`
 

## Languages

- Ada (Generic)
- C/C++
- Fortran (Generic)
- Haskell
- Julia   (Generic)
- Python
- Rust (for simplicity if you already have the library)

## Performance
Machine-prime is fast enough that even with the overhead of dynamic library calls, it will almost certainly be faster than any native
implementation of a primality test (excepting identical implementations of machine-prime). Observe that machine-prime's C binding is 
at minimum 3x faster than Forisek and Jancina's native C implementation (FJ262K). Even python's abysmal performance is sufficient to outspeed the 
FJ262K algorithm. 

Machine-prime has 3 modes Default, Small, and Tiny. Default is the fastest, and Tiny is the slowest. All of the bindings are benchmarked using the Default mode dynamic library on Linux. Two benchmarks are used: is_prime against the largest 100 million integers, and is_prime_wc against 2^64-59 the hardest case, 10 million times. Additionally we compare against Forisek & Jancina's implementation which was the prior fastest implementation. 
The benchmarks here are taken on an Intel i5-5300U (4) @ 2.900GHz

- Native Rust Default: Average 6s Worst 7.5s
- Native Rust Tiny: Average 21.4s Worst 9.6s
- Ada: Average 6.4s Worst 8.1s
- C/Fortran/Rust: Average 6s Worst 7.5s
- Julia: Average 6s Worst 7.5s (approximately 8x faster than Primes Julia pkg)
- Python: Average 32s Worst 9.7s
- F&J C Native: Average 48s  Worst 27s


While the Tiny mode library has not been benchmarked, it is easy to see that it will still be faster than FJ262K since most of the
difference is in constant time calls. The Python `for range` used in the Average benchmark appears to be exceptionally slow. 
