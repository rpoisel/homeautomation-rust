all: native

.PHONY: prepare
prepare:
	if [ ! -e build.venv ]; then python -m venv build.venv; fi
	(cd build.venv \
		&& . bin/activate \
		&& pip install conan\
		&& if ! conan profile show default > /dev/null 2>&1; then \
			conan profile new default --detect \
				&& conan profile update settings.compiler.libcxx=libstdc++11 default; \
			fi)

# TODO define dependencies between native and native-conan
native-conan: prepare
	if [ ! -e build ]; then mkdir build; fi
	cd build \
		&& . ../build.venv/bin/activate \
		&& conan profile update settings.compiler.libcxx=libstdc++11 default \
		&& conan install --build=paho-mqtt-c --build=paho-mqtt-cpp ..

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
		&& . ../build.venv/bin/activate \
		&& (conan install --build=paho-mqtt-c --build=paho-mqtt-cpp .. || true) \
		&& cmake -DCMAKE_BUILD_TYPE=RelMinSize -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_TOOLCHAIN_FILE=../cmake/toolchain/toolchain-aarch64-rpi4.cmake -GNinja .. \
		&& ninja -v

.PHONY: raspberry-deploy
raspberry-deploy: raspberry
	scp -O build.rasppi/src/roof root@raspberry-d.lan:/tmp

.PHONY: clean
clean:
	rm -rf build.venv build build.rasppi
