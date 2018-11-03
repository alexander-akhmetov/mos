NASM=/usr/local/bin/nasm
QEMU=qemu-system-x86_64
LD=~/opt/cross/bin/x86_64-elf-ld

BUILD_DIR=_build/
RUST_BUILD_DIR=target


install-requirements:
	# rust nightly
	cargo install cargo-xbuild
	rustup component add rust-src

	cargo install bootimage --version "^0.5.0"

clean:
	rm -rf $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)

build-bootloader: clean
	$(NASM) -f elf64 src/boot/multiboot_header.asm -o $(BUILD_DIR)/multiboot_header.o
	$(NASM) -f elf64 src/boot/boot.asm -o $(BUILD_DIR)/boot.o
	$(LD) --nmagic \
			-o $(BUILD_DIR)/kernel.bin \
			-T src/boot/linker.ld \
			$(BUILD_DIR)/multiboot_header.o \
			$(BUILD_DIR)/boot.o


build-kernel:
	cargo xbuild --target x86_64-mos.json


build:
	make build-kernel
	make build-bootloader


build/%:
	cargo xbuild --target x86_64-mos.json --bin $*

iso: build
	mkdir -p $(BUILD_DIR)/isofiles/boot/grub
	cp src/boot/grub.cfg $(BUILD_DIR)/isofiles/boot/grub
	cp $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/isofiles/boot

	docker-compose run build_os grub-mkrescue -o /src/$(BUILD_DIR)/os.iso /src/$(BUILD_DIR)/isofiles


unit-tests:
	cargo test

tests: unit-tests integration-tests

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


qemu-run:
	# qemu-system-x86_64 -drive format=raw,file=target/x86_64-mos/debug/bootimage-mos.bin
	$(QEMU) -cdrom $(BUILD_DIR)/os.iso

