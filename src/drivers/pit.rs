use cpuio;

const PIT_CONTROL_PORT: u16 = 0x43;
const PIT_DATA_PORT: u16 = 0x40;

pub fn init(frequency: u32) {
    unsafe {
        let divisor: u32 = 1193180 / frequency;

        let mut control_port: cpuio::UnsafePort<u8> = cpuio::UnsafePort::new(PIT_CONTROL_PORT);
        let mut channel_port: cpuio::UnsafePort<u8> = cpuio::UnsafePort::new(PIT_DATA_PORT);

        control_port.write(0x36);
        channel_port.write((divisor & 0xFF) as u8);
        channel_port.write((divisor >> 8 & 0xFF) as u8);
    }
}
