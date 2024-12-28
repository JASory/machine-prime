@echo off
 mkdir libprime
 echo Select 1  for Default, 2 for Lucas, 3  for Table, 4 for SSMR 
 set /p flag=""
 if %flag%==1 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc
 echo Built the Default variant)
 if %flag%==2 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc --cfg feature=\"lucas\"
 echo Built the Lucas variant) 
 if %flag%==3 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc --cfg feature=\"table\"
 echo Built the Table variant)
 if %flag%==3 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc --cfg feature=\"ssmr\"
 echo Built the SSMR variant) 
pause
