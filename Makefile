install:
	cargo build --release
	sudo cp target/release/guse /usr/local/bin/
