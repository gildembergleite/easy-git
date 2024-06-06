build-linux:
	cargo build --release
	
build-windows:
	cargo build --release --target x86_64-pc-windows-gnu

run-linux:
	./target/release/easy-git

run-windows:
	./target/x86_64-pc-windows-gnu/release/easy-git.exe
	