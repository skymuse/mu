PROGRAM := mu
TARGET := ./target/release/${PROGRAM}

all: release

release:
	cargo build --release
install: ${TARGET}
	bin install ${TARGET}
uninstall:
	bin uninstall ${PROGRAM}
clean:
	cargo clean
format:
	cargo fmt
