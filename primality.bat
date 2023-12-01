@echo off
 mkdir libprime
 echo Select 1  for Default, 2 for Small, or 3  for Tiny
 set /p flag=""
 if %flag%==1 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc
 echo Built the Default variant)
 if %flag%==2 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc --cfg feature=\"small\"
 echo Built the Small variant) 
 if %flag%==3 (rustc -C opt-level="3" src/lib.rs -o libprime/prime.dll --crate-type cdylib --target x86_64-pc-windows-msvc --cfg feature=\"tiny\"
 echo Built the Tiny variant)
pause
