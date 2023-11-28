.PHONY: fmt clippy clean build pack all ci

all: clean fmt clippy pack

ci: fmt clippy

fmt:
	cargo fmt --all --

clippy:
	cargo clippy --all -- -D warnings

clean:
	rm -rf ./target && rm -rf ./*/target

build:
	cargo build --all

pack:
	cargo build --all --release
