machineprime: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu
	strip libprime.so
	
small: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="small"'
	strip libprime.so

tiny: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="tiny"'
	strip libprime.so
	
ssmr: src/lib.rs
	rustc -C opt-level="3" src/lib.rs -o libprime.so --crate-type cdylib --target x86_64-unknown-linux-gnu --cfg 'feature="ssmr"'
	strip libprime.so
	
install: libprime.so
	install libprime.so /lib/libprime.so
	
install-local: libprime.so
	install libprime.so /usr/local/lib/libprime.so	
