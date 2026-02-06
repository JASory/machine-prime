# Changelog


## Version 1.5.6

API changes

### Added
- QFT feature. is_prime_128 can now use Kashin's QFT plus a witness-2 strong fermat test

### Removed
 Table feature, SSMR is now the default 
### Changes
- Internal API montgomery arithmetic now places the inverse before the ring e.g montprod(x,y,inv,ring) instead of montprod(x,y,ring,inv)
- Multiplicative inverse trial division is used by is_prime_128 and the entire 64-bit range for is_prime. Previously this was only for n under  2^64/331
- The Lucas test now uses isqrt to eliminate any possible perfect squares rather than checking the few known cases. is_prime_wc_128 is now guaranteed to finish, previously it was only conjectured,
- Lucas' test truncated Jacobi symbol calculates slightly faster
- mulinv2_128 calculates slightly faster   
