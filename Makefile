all: release

release:
	cargo build --release
clean:
	cargo clean
format:
	cargo fmt
