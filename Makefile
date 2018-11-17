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
	rm -rf $(RUST_BUILD_DIR)


build-bootloader:
	mkdir -p $(BUILD_DIR)

	$(NASM) -f elf64 src/boot/loader/multiboot_header.asm -o $(BUILD_DIR)/multiboot_header.o
	$(NASM) -f elf64 src/boot/loader/boot.asm -o $(BUILD_DIR)/boot.o
	$(NASM) -f elf64 src/boot/loader/long_mode_init.asm -o $(BUILD_DIR)/long_mode_init.o
	$(NASM) -f elf64 src/boot/loader/checks.asm -o $(BUILD_DIR)/checks.o
	$(NASM) -f elf64 src/multitasking/switch_to.asm -o $(BUILD_DIR)/switch_to.o
	$(LD) --nmagic \
			-o $(BUILD_DIR)/kernel.bin \
			-T src/boot/loader/linker.ld \
			$(BUILD_DIR)/multiboot_header.o \
			$(BUILD_DIR)/boot.o \
			$(BUILD_DIR)/long_mode_init.o \
			$(BUILD_DIR)/checks.o \
			$(BUILD_DIR)/switch_to.o \
			$(RUST_BUILD_DIR)/x86_64-mos/debug/libmos.a


build-kernel:
	cargo xbuild --target x86_64-mos.json


build-os-binaries:
	cd ./lib/userspace/msh && make build
	cp ./lib/userspace/msh/target/x86_64-msh/debug/msh ./initrd/msh

	cd ./lib/userspace/hello_world && make build
	cp ./lib/userspace/hello_world/target/x86_64-hello_world/debug/hello_world ./initrd/hello_world


build-initrd:
	mkdir -p $(BUILD_DIR)/isofiles/boot/
	cd ./initrd/ && tar --format ustar -c * > ../$(BUILD_DIR)/isofiles/boot/initrd.tar

build:
	mkdir -p $(BUILD_DIR)
	make build-kernel
	make build-bootloader
	make build-os-binaries
	make build-initrd


build/%:
	cargo xbuild --target x86_64-mos.json --bin $*

iso: build
	# copy loader
	mkdir -p $(BUILD_DIR)/isofiles/boot/grub
	cp src/boot/loader/grub.cfg $(BUILD_DIR)/isofiles/boot/grub
	cp $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/isofiles/boot

	docker-compose run build_os grub-mkrescue -o /src/$(BUILD_DIR)/os.iso /src/$(BUILD_DIR)/isofiles


unit-tests:
	cargo test
	cd lib/librust && cargo test

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
	$(QEMU) -cdrom $(BUILD_DIR)/os.iso \
		-serial mon:stdio \
		-m 512M \
		-boot d \
		# -d int \
		# -monitor stdio
