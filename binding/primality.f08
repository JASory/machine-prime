! gfortran -c primality.f08 -lprime
! gfortran file.f95 primality.o -lprime
! 

MODULE PRIMALITY
 
 USE, INTRINSIC :: iso_c_binding
 IMPLICIT NONE
 
 
 INTERFACE IS_PRIME

 Logical(Kind = C_Bool) function is_prime_32(x) Bind(C, name = "is_prime")
   use, intrinsic :: iso_c_binding
   Integer(Kind = C_INT32_T), Intent(in), Value :: x
 End function is_prime_32 
 
 Logical(Kind = C_Bool) function is_prime_64(x) Bind(C, name = "is_prime")
   use, intrinsic :: iso_c_binding
   Integer(Kind = C_INT64_T), Intent(in), Value :: x
 End function is_prime_64 
 
 Logical(Kind = C_Bool) function is_prime_128(x) Bind(C, name = "is_prime_128")
   use, intrinsic :: iso_c_binding
   Integer(Kind = C_INT128_T), Intent(in), Value :: x
 End function is_prime_128 

 End INTERFACE IS_PRIME
 
 INTERFACE IS_PRIME_WC

 Logical(Kind = C_Bool) function is_prime_wc_32(x) Bind(C, name = "is_prime_wc")
   use, intrinsic :: iso_c_binding
   Integer(Kind = C_INT32_T), Intent(in), Value :: x
 End function is_prime_wc_32 
 
 Logical(Kind = C_Bool) function is_prime_wc_64(x) Bind(C, name = "is_prime_wc")
   use, intrinsic :: iso_c_binding
   Integer(Kind = C_INT64_T), Intent(in), Value :: x
 End function is_prime_wc_64 

 Logical(Kind = C_Bool) function is_prime_wc_128(x) Bind(C, name = "is_prime_wc_128")
   use, intrinsic :: iso_c_binding
   Integer(Kind = C_INT128_T), Intent(in), Value :: x
 End function is_prime_wc_128
 
 End INTERFACE IS_PRIME_WC
 
 End Module PRIMALITY
