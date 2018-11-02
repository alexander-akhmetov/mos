NASM=/usr/local/bin/nasm
LD=~/opt/cross/bin/x86_64-elf-ld
BUILD_DIR=_build/


install-requirements:
	# rust nightly
	cargo install cargo-xbuild
	rustup component add rust-src

	cargo install bootimage --version "^0.5.0"


clean:
	rm -rf $(BUILD_DIR)
	mkdir -p $(BUILD_DIR)

build-bootloader: clean
	$(NASM) -f elf64 src/bootsector/multiboot_header.asm -o $(BUILD_DIR)/multiboot_header.o
	$(NASM) -f elf64 src/bootsector/boot.asm -o $(BUILD_DIR)/boot.o
	$(LD) --nmagic \
			-o $(BUILD_DIR)/kernel.bin \
			-T src/bootsector/linker.ld \
			$(BUILD_DIR)/multiboot_header.o \
			$(BUILD_DIR)/boot.o


build-kernel:
	# cargo xbuild --target x86_64-mos.json

	# the next command uses previous command and also creates a bootimage
	#
	# bootimage command: https://github.com/rust-osdev/bootimage
	# bootloader: https://github.com/rust-osdev/bootloader
	bootimage build


iso:
	mkdir -p $(BUILD_DIR)/isofiles/boot/grub
	cp src/grub/grub.cfg $(BUILD_DIR)/isofiles/boot/grub
	cp $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/isofiles/boot

	docker-compose run build_os grub-mkrescue -o /src/$(BUILD_DIR)/os.iso /src/$(BUILD_DIR)/isofiles


qemu-run:
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-mos/debug/bootimage-mos.bin
