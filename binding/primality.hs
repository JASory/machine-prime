-- Glasgow Haskell compiler 
-- ghc primality.hs -lprime

foreign import ccall "is_prime" is_prime :: Word -> Bool
foreign import ccall "is_prime_wc" is_prime_wc :: Word -> Bool
