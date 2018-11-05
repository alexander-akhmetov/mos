use cpuio;
use spin::Mutex;

mod time;

lazy_static! {
    pub static ref CMOS_WRITER: Mutex<cpuio::Port<u8>> =
        Mutex::new(unsafe { cpuio::Port::new(0x70) });
    pub static ref CMOS_READER: Mutex<cpuio::Port<u8>> =
        Mutex::new(unsafe { cpuio::Port::new(0x71) });
}

pub fn get_timestamp() -> u32 {
    let seconds = bcd_to_binary(cmos_read(0));
    let minutes = bcd_to_binary(cmos_read(2));
    let hour = bcd_to_binary(cmos_read(4));
    let day = bcd_to_binary(cmos_read(7));
    let month = bcd_to_binary(cmos_read(8));
    let year = bcd_to_binary(cmos_read(9));

    time::mk_timestamp(seconds, minutes, hour, day, month, year)
}

fn cmos_read(addr: u8) -> u8 {
    // 0x80 means NMI is disabled
    CMOS_WRITER.lock().write(0x80 | addr);
    CMOS_READER.lock().read()
}

fn bcd_to_binary(value: u8) -> u8 {
    // Binary Coded Decimal to binary
    // #define BCD_TO_BIN(val) ((val)=((val)&15) + ((val)>>4)*10)  // linux 0.0.1
    (value & 15) + (value >> 4) * 10
}
