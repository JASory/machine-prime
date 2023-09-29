package Machine_Prime.Primality is 


generic type T is mod <>;
  function is_prime(X : T) return Boolean;

generic type T is mod <>;
  function is_prime_wc(X : T) return Boolean;

end Machine_Prime.Primality;
