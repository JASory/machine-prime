These are bindings to machine-prime for various languages. Machine-prime is fast enough that even with library call overhead, 
it is almost surely going to be faster than any other existing primality test written in that language. Consider that machine-prime's C binding is 4x faster than Forisek & Jancina's implementation written in native C.

Each file is for a different language. They contain instructions on how to compile and link to the machine-prime library. 
These all assume that it was compiled and saved as `/lib/libprime.so`
 
It is only supported up to 2^64, all values beyond that are erroneous. 

## Languages

- C/C++
- Fortran
- Julia
- Python
- Rust (for simplicity if you already have the library)

## Performance
Machine-prime has 3 modes Default, Small, and Tiny. Default is the fastest, and Tiny is the slowest. All of the bindings are benchmarked using the Default mode dynamic library on Linux. Two benchmarks are used: is_prime against the largest 100 million integers, and is_prime_wc against 2^64-59 the hardest case, 10 million times. Additionally we compare against Forisek & Jancina's implementation which was the prior fastest implementation. 

- Native Rust Default: Average 5s Worst 6.36
- Native Rust Tiny: Average 17.13s Worst 7.25s
- C/Fortran/Rust: Average 5s Worst 6s
- Python: Average 32s Worst 9.7s.
- F&J C Native: Average 40s  Worst 22s

While the Tiny mode library has not been benchmarked, it is easy to see that it will still be faster than FJ262K since most of the
difference is in constant time calls. The Python `for range` used in the Average benchmark appears to be exceptionally slow. 
