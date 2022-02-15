ifeq (, $(shell command -v rustup))
	$(error "No rustup in $(PATH)")
endif

all: build-native build-rasppi

.PHONY: build-native
build-native:
	cargo build --release
	strip -s target/release/homeautomation-rust
	size target/release/homeautomation-rust

.PHONY: build-rasppi
build-rasppi:
	cargo build --target=armv7-unknown-linux-gnueabihf --release
	arm-linux-gnueabihf-strip -s target/armv7-unknown-linux-gnueabihf/release/homeautomation-rust
	arm-linux-gnueabihf-size target/armv7-unknown-linux-gnueabihf/release/homeautomation-rust

.PHONY: prepare
prepare:
	rustup target add armv7-unknown-linux-gnueabihf
	sudo apt install gcc-arm-linux-gnueabihf
