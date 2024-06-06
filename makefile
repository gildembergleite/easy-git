build:
	rm -rf ./target && \
	cargo build --release && \
	cargo build --release --target x86_64-pc-windows-gnu

run:
	./target/release/easy-git