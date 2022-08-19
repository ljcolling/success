default :
	cargo build --release
	cp ./target/release/success /usr/local/bin
