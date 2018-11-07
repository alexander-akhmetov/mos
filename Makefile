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
	rm -rf $(RUST_BUILD_DIR)


build-bootloader:
	$(NASM) -f elf64 src/boot/loader/multiboot_header.asm -o $(BUILD_DIR)/multiboot_header.o
	$(NASM) -f elf64 src/boot/loader/boot.asm -o $(BUILD_DIR)/boot.o
	$(NASM) -f elf64 src/boot/loader/long_mode_init.asm -o $(BUILD_DIR)/long_mode_init.o
	$(NASM) -f elf64 src/boot/loader/checks.asm -o $(BUILD_DIR)/checks.o
	$(LD) --nmagic \
			-o $(BUILD_DIR)/kernel.bin \
			-T src/boot/loader/linker.ld \
			$(BUILD_DIR)/multiboot_header.o \
			$(BUILD_DIR)/boot.o \
			$(BUILD_DIR)/long_mode_init.o \
			$(BUILD_DIR)/checks.o \
			$(RUST_BUILD_DIR)/x86_64-mos/debug/libmos.a


build-kernel:
	cargo xbuild --target x86_64-mos.json


build-hello-asm:
	$(NASM) -f bin src/boot/loader/hello.asm -o $(BUILD_DIR)/hello.bin

build-initrd:
	cd ./src/boot/initrd/ && tar --format ustar -c * > ../../../$(BUILD_DIR)/isofiles/boot/initrd

build:
	make build-kernel
	make build-bootloader
	make build-hello-asm
	make build-initrd


build/%:
	cargo xbuild --target x86_64-mos.json --bin $*

iso: build
	# copy loader
	mkdir -p $(BUILD_DIR)/isofiles/boot/grub
	cp src/boot/loader/grub.cfg $(BUILD_DIR)/isofiles/boot/grub
	cp $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/isofiles/boot

	# copy modules
	cp $(BUILD_DIR)/hello.bin $(BUILD_DIR)/isofiles/boot/hello

	docker-compose run build_os grub-mkrescue -o /src/$(BUILD_DIR)/os.iso /src/$(BUILD_DIR)/isofiles


unit-tests:
	cargo test

tests: unit-tests


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


qemu-run: iso
	# mkdir -p _floppy

	# -boot d - boot from cdrom first
	$(QEMU) -cdrom $(BUILD_DIR)/os.iso \
		-serial mon:stdio \
		-m 512M \
		-boot d \
		# -fda fat:r:floppy:./_floppy
