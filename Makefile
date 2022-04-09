all:

ifeq (, $(shell command -v rustup))
	$(error "No rustup in $(PATH)")
endif

.PHONY: prepare
prepare:
	rustup target add armv7-unknown-linux-gnueabihf
	sudo apt install gcc-arm-linux-gnueabihf

.PHONY: build-native
build-native:
	cargo build --release
	@for i in cellar roof; do \
		strip -s "target/release/$$i"; \
		size "target/release/$$i"; \
	done

.PHONY: build-rasppi
all: build-rasppi
build-rasppi:
	cargo build --target=armv7-unknown-linux-gnueabihf --release
	@for i in cellar roof; do \
		arm-linux-gnueabihf-strip -s "target/armv7-unknown-linux-gnueabihf/release/$$i"; \
		arm-linux-gnueabihf-size "target/armv7-unknown-linux-gnueabihf/release/$$i"; \
	done
