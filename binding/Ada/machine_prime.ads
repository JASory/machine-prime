with Interfaces.C; use Interfaces.C;


package Machine_Prime is 

private

function mprime(x: unsigned_long) return unsigned_char
   with 
     Import => True,
     Convention => C,
     External_Name => "is_prime"; 
     
function mprime_wc(x: unsigned_long) return unsigned_char
   with 
     Import => True,
     Convention => C,
     External_Name => "is_prime_wc";    
     
end Machine_Prime;       
