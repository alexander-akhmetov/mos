# OS

* QEMU: `qemu-system-x86_64`
* Always use `/usr/local/bin/nasm`

## Bootsector

When the computer boots, the BIOS doesn't know how to load the OS, so it delegates that task to the boot sector. Thus, the boot sector must be placed in a known, standard location. That location is the first sector of the disk (cylinder 0, head 0, sector 0) and it takes 512 bytes.

To make sure that the "disk is bootable", the BIOS checks that bytes 511 and 512 of the alleged boot sector are bytes 0xAA55.

To make sure that the "disk is bootable", the BIOS checks that bytes 511 and 512 of the alleged boot sector are bytes 0xAA55.

## GCC cross-compiler

* [binutils installation](https://os.phil-opp.com/cross-compile-binutils/)
* [grub on macos](https://github.com/phil-opp/blog_os/issues/55)
* [more about grub](https://gist.github.com/alexander-akhmetov/f0a07f2264047a746907c3cfa182ae81)
