use core::marker::PhantomData;

mod x86;

pub struct Port<T: InOut> {
    // address
    port: u16,
    // https://doc.rust-lang.org/core/marker/struct.PhantomData.html
    phantom: PhantomData<T>,
}

pub trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 {
        x86::inb(port)
    }
    unsafe fn port_out(port: u16, value: u8) {
        x86::outb(value, port);
    }
}

impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 {
        x86::inw(port)
    }
    unsafe fn port_out(port: u16, value: u16) {
        x86::outw(value, port);
    }
}

impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 {
        x86::inl(port)
    }
    unsafe fn port_out(port: u16, value: u32) {
        x86::outl(value, port);
    }
}

impl<T: InOut> Port<T> {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            phantom: PhantomData,
        }
    }

    pub fn read(&mut self) -> T {
        unsafe { T::port_in(self.port) }
    }

    pub fn write(&mut self, value: T) {
        unsafe {
            T::port_out(self.port, value);
        }
    }
}

pub struct UnsafePort<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> UnsafePort<T> {
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort {
            port: port,
            phantom: PhantomData,
        }
    }

    pub unsafe fn read(&mut self) -> T {
        T::port_in(self.port)
    }

    pub unsafe fn write(&mut self, value: T) {
        T::port_out(self.port, value);
    }
}
