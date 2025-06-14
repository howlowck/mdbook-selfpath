all: build
check: build clippy test install compare
build:
	cargo build
clippy:
	cargo clippy
test:
	cargo test
install:
	cargo install --path .