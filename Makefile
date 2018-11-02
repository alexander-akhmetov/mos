BUILD_DIR=target


install-requirements:
	# rust nightly
	cargo install cargo-xbuild
	rustup component add rust-src

	cargo install bootimage --version "^0.5.0"

clean:
	rm -rf $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)

build:
	# cargo xbuild --target x86_64-mos.json

	# the next command uses previous command and also creates a bootimage
	#
	# bootimage command: https://github.com/rust-osdev/bootimage
	# bootloader: https://github.com/rust-osdev/bootloader
	bootimage build

build/%:
	bootimage build --bin $*

test:
	cargo test

tests: test

integration-test/%:
	make build/$*

	-qemu-system-x86_64 \
		-serial mon:stdio \
		-drive format=raw,file=target/x86_64-mos/debug/bootimage-$*.bin \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-display none
	exit 0

integration-tests:
	bash run-tests.sh


qemu-run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-mos/debug/bootimage-mos.bin

