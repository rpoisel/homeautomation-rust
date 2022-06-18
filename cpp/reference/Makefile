all: native

.PHONY: native
native:
	if [ ! -e build ]; then mkdir build; fi
	cd build \
	  && cmake -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_TOOLCHAIN_FILE=../cmake/toolchain/toolchain-native.cmake -GNinja .. \
	  && ninja -v
	ln -sf build/compile_commands.json

.PHONY: test
test: native
	cd build && ctest --verbose

.PHONY: raspberry
raspberry:
	if [ ! -e build.rasppi ]; then mkdir build.rasppi; fi
	cd build.rasppi \
	  && cmake -DCMAKE_BUILD_TYPE=RelMinSize -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_TOOLCHAIN_FILE=../cmake/toolchain/toolchain-aarch64-rpi4.cmake -GNinja .. \
	  && ninja -v

.PHONY: raspberry-deploy
raspberry-deploy: raspberry
	scp -O build.rasppi/src/roof root@raspberry-d.lan:/tmp

.PHONY: clean
clean:
	rm -rf build build.rasppi
