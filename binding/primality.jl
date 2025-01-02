# Julia Bindings for machine-prime
# May accept any Unsigned integer but only accurately evaluates  under 2^64 

# include("primality.jl")

function is_prime(x::Unsigned)
   y = UInt64(x);
   return ccall( (:is_prime_128, "libprime"), Bool, (UInt128,), y)
end    
 
function is_prime_wc(x::Unsigned)
   y = UInt64(x);
   return ccall( (:is_prime_wc_128, "libprime"), Bool, (UInt128,), y)
end    

#function is_prime(x::Unsigned)
#   y = UInt128(x);
#   return ccall( (:is_prime_128, "libprime"), Bool, (UInt128,), y)
#end    
 
#function is_prime_wc(x::Unsigned)
#   y = UInt128(x);
#   return ccall( (:is_prime_wc_128, "libprime"), Bool, (UInt128,), y)
#end    



