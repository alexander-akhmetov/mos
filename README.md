# mos

My attempt to write a small operating system (and to try Rust :) ).

**mos**

![mos: msh](https://github.com/alexander-akhmetov/mos/blob/readme-pics/screenshots/msh.png?raw=true)

**mOS boot log**

![mos: start-up logs](https://github.com/alexander-akhmetov/mos/blob/readme-pics/screenshots/start.png?raw=true)



## Development

Start QEMU with mOS inside:

```shell
make run
```

Run tests:

```shell
make tests
```

### Folders structure

* `./initrd`: all files in this directory will be placed into a single archive and mounted as a TarFS to `/initrd`
* `./src`: kernel source code
* `./lib`: Contains userspace programs and `librust`: Rust library to write programs for mOS.

**OS Entrypoint**: `/src/lib.rs`.

## Requirements

* Cross compiled binutils (actually only `ld`).
* NASM
* Rust nightly (I used `1.31.0-nightly (d586d5d2f 2018-10-29)`)
* Docker to build iso with GRUB bootloader
* QEMU

## librust

You can build userspace programs for mOS. To communicate with OS from Rust code you need to use `librust`.
It is a library which provides interface to use all implemented system calls.

# What can it do?

* [x] Load! :-)
* [x] Print to VGA buffer
* [x] Mount ramdisk
* [x] Built-in support for TarFS
* [x] Interrupts
* [x] System clock
* [x] Syscalls ABI
* [x] Processes support, context switching
* [x] Start init process
* [x] Shell for user's commands: `msh`


## ToDo

* Fix paging. It does not work properly, and because of that it's not possible to start separate processes. Now programs are being running as threads inside the OS.
* After paging mmap call in the `librust` must be completed, so it will be possible to request memory from userspace.
* Higher Half Kernel
* `fork` syscall
...
