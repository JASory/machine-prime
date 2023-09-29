-- Call using   gnatmake yourprogram.adb -largs -lprime
-- Produces a generic primality function over all modular types

with Interfaces.C; use Interfaces.C;
with Machine_Prime; use Machine_Prime;

package body Machine_Prime.Primality is

function is_prime(X : T) return Boolean is
 begin 

   if mprime(unsigned_long(X)) = 0 then 
       return False;
  else
       return True;
  end if; 
end is_prime;

function is_prime_wc(X : T) return Boolean is
 begin 

   if mprime_wc(unsigned_long(X)) = 0 then 
       return False;
  else
       return True;
  end if; 
end is_prime_wc;

end Machine_Prime.Primality;
