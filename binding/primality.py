# python3 script.py

from ctypes import *

primality = CDLL("libprime.so")

primality.is_prime.argtype=c_ulonglong
primality.is_prime.restype=c_bool

primality.is_prime.argtype=c_ulonglong
primality.is_prime.restype=c_bool

#                Benchmark

#   This benchmark is identical to the machine-prime crates, apparently Python's for range is extremely slow
 
#            Native Rust      Python Binding
# Top 10^8    5.08s                  31.9s
# Strongest   6.379s                  9.673s

