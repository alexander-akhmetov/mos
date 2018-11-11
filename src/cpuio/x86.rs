/*

    Wrappers around the x86-family I/O instructions
    to work with CPI I/O ports.

*/

pub unsafe fn inb(port: u16) -> u8 {
    /// reads a `u8`-sized value from `port`
    //
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outb(value: u8, port: u16) {
    /// writes a `u8`-sized `value` to `port`
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

pub unsafe fn inw(port: u16) -> u16 {
    /// read a `u16`-sized value from `port`
    let result: u16;
    asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outw(value: u16, port: u16) {
    // writes a `u8`-sized `value` to `port`
    asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
}

pub unsafe fn inl(port: u16) -> u32 {
    /// reads a `u32`-sized value from `port`
    let result: u32;
    asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    result
}

pub unsafe fn outl(value: u32, port: u16) {
    /// writes a `u32`-sized `value` to `port`
    asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
}
