use crate::cpuio;

const DEFAULT_FREQ: u32 = 1_193_180;
const PIT_CONTROL_PORT: u16 = 0x43;
const PIT_DATA_PORT: u16 = 0x40;

pub fn init(frequency: u32) {
    /// sets PIT frequency to "frequency"
    unsafe {
        // by default PIT generates a signal with 1.193182 MHz frequency
        // it's configurable, if we want to change this, we can set a "divisor",
        // so signal will be triggered every 1.193182/divisor
        let divisor: u32 = DEFAULT_FREQ / frequency;

        let mut control_port: cpuio::UnsafePort<u8> = cpuio::UnsafePort::new(PIT_CONTROL_PORT);
        let mut channel_port: cpuio::UnsafePort<u8> = cpuio::UnsafePort::new(PIT_DATA_PORT);

        // send "start configuring" command
        control_port.write(0x36);
        // send low bytes of divisor
        channel_port.write((divisor & 0xFF) as u8);
        // send high bytes of divisor
        channel_port.write((divisor >> 8 & 0xFF) as u8);
    }
}
