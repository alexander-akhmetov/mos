# OS

* QEMU: `qemu-system-x86_64`
* Always use `/usr/local/bin/nasm`

## Development

Start QEMU with mOS inside:

```shell
make run
```

### Folders structure

* `./initrd`: all files in this directory will be placed into a single archive and mounted as a TarFS to `/initrd`
* `./src`: kernel source code

## Requirements

* Cross compiled binutils (actually only `ld`).
* NASM
* Rust nightly
* Docker to build iso with GRUB bootloader
* QEMU

### Requirements installation

nasm:

```shell
brew install nasm
```

[Binutils installation](https://os.phil-opp.com/cross-compile-binutils/)

# What it can?

* [x] Load! :-)
* [x] Print to VGA buffer
* [x] Mount ramdisk
* [x] Built-in support for TarFS
* [x] Interrupts
* [x] System clock
* [x] Syscalls ABI
* [ ] Processes
* [ ] Start init process
* [ ] Shell for user's commands
* [ ] `cat`, `ps`
