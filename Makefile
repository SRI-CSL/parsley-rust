.PHONY: all release clean test

all:
	cargo build

release:
	cargo build --release

test:
	cargo test

clean:
	cargo clean

docker: release etc/docker/Dockerfile
	docker build -t 'pdf_printer' -f etc/docker/Dockerfile .
