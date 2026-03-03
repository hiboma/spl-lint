.PHONY: build test lint clean install check all

all: lint test build

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

fmt:
	cargo fmt

clean:
	cargo clean

install:
	cargo install --path .

check: lint test
