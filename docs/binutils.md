# Cross Compile Binutils

Download `binutils` [here](https://os.phil-opp.com/cross-compile-binutils/).

Unpack:

```shell
tar xf binutils-2.31.90.tar.xz
```

Configure:

```shell
cd binutils-2.31.90

../binutils-2.31.90/configure --target=x86_64-elf --prefix="$HOME/opt/cross" \
    --disable-nls --disable-werror \
    --disable-gdb --disable-libdecnumber --disable-readline --disable-sim
```

Build and install:

```shell
make
make install
```

Finally, add the `bin` directory to the `PATH`:

```shell
export PATH="$HOME/opt/cross/bin:$PATH"
```
