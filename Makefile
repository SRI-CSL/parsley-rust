.PHONY: all clean test

all:
	cargo build

test:
	cargo test

clean:
	cargo clean
