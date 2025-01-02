#include<stdint.h>
#include<stdbool.h>

 // Install to /usr/include/ to use as C library  #include "primality.h"
 // Install to /usr/include/cpp/10/ to use  #include<primality.h> with C++
 // Optionally one may rename this to "primality" for a cleaner look
 // gcc file.c -lprime  or g++ file.cpp -lprime

#ifdef __cplusplus
extern "C" {
#endif

 bool is_prime(uint64_t x);

 bool is_prime_wc(uint64_t x);
 
/* 
 bool is_prime_128(__int128 x);
 
 bool is_prime_wc_128(__int128 x);
*/
#ifdef __cplusplus
}  
#endif
