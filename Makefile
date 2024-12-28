machineprime: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu
	strip libprime.so
	
lucas: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="lucas"'
	strip libprime.so

table: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="table"'
	strip libprime.so
	
ssmr: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="ssmr"'
	strip libprime.so
	
wide: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="wide"'
	strip libprime.so

lucaswide: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="wide"' --cfg 'feature="lucas"'
	strip libprime.so

tablewide: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="wide"' --cfg 'feature="table"'
	strip libprime.so

ssmrwide: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="wide"' --cfg 'feature="ssmr"'
	strip libprime.so

install: libprime.so
	install libprime.so /lib/libprime.so
	
install-local: libprime.so
	install libprime.so /usr/local/lib/libprime.so	
