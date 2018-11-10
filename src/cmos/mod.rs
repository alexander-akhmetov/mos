use cpuio;
use spin::Mutex;

mod time;

enum CMOSAddr {
    Writer = 0x70,
    Reader = 0x71,
}

enum CMOSRegister {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    Day = 0x07,
    Month = 0x08,
    Year = 0x09,
    Status = 0x0A,
}

lazy_static! {
    pub static ref CMOS_WRITER: Mutex<cpuio::Port<u8>> =
        Mutex::new(unsafe { cpuio::Port::new(CMOSAddr::Writer as u16) });
    pub static ref CMOS_READER: Mutex<cpuio::Port<u8>> =
        Mutex::new(unsafe { cpuio::Port::new(CMOSAddr::Reader as u16) });
}

pub fn get_timestamp() -> u64 {
    let datetime = get_datetime();
    datetime.timestamp()
}

pub fn get_datetime() -> time::DateTime {
    // while CMOS in update process, wait
    while is_updating() {}

    let seconds = bcd_to_binary(cmos_read(CMOSRegister::Seconds));
    let minutes = bcd_to_binary(cmos_read(CMOSRegister::Minutes));
    let hour = bcd_to_binary(cmos_read(CMOSRegister::Hours));
    let day = bcd_to_binary(cmos_read(CMOSRegister::Day));
    let month = bcd_to_binary(cmos_read(CMOSRegister::Month));
    let year = bcd_to_binary(cmos_read(CMOSRegister::Year));

    time::DateTime {
        seconds: seconds,
        minutes: minutes,
        hour: hour,
        day: day,
        month: month,
        year: year,
    }
}

fn cmos_read(addr: CMOSRegister) -> u8 {
    // 0x80 means NMI is disabled
    CMOS_WRITER.lock().write(0x80 | addr as u8);
    CMOS_READER.lock().read()
}

fn bcd_to_binary(value: u8) -> u8 {
    // Binary Coded Decimal to binary
    // #define BCD_TO_BIN(val) ((val)=((val)&15) + ((val)>>4)*10)  // linux 0.0.1
    (value & 15) + (value >> 4) * 10
}

fn is_updating() -> bool {
    let value = cmos_read(CMOSRegister::Status);

    (value & 0x80) > 0
}
