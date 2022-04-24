all:

ifeq (, $(shell command -v rustup))
	$(error "No rustup in $(PATH)")
endif

.PHONY: prepare
prepare:
	sudo apt install -yy gcc-arm-linux-gnueabihf gcc-aarch64-linux-gnu
	rustup target add armv7-unknown-linux-gnueabihf
	rustup target add aarch64-unknown-linux-gnu

.PHONY: test
test:
	cargo test

.PHONY: build-native
all: build-native
build-native:
	cargo build --release
	@for i in cellar roof; do \
		strip -s "target/release/$$i"; \
		size "target/release/$$i"; \
	done

.PHONY: build-rasppi
all: build-rasppi
build-rasppi:
	cargo build --target=aarch64-unknown-linux-gnu --release
	@for i in cellar roof; do \
		aarch64-linux-gnu-strip -s "target/aarch64-unknown-linux-gnu/release/$$i"; \
		aarch64-linux-gnu-size "target/aarch64-unknown-linux-gnu/release/$$i"; \
	done
