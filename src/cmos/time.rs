const MINUTE: u32 = 60;
const HOUR: u32 = MINUTE * 60;
const DAY: u32 = HOUR * 24;
const YEAR: u32 = DAY * 365;
static MONTH: [u32; 12] = [
	0,
	DAY*(31),
	DAY*(31+29),
	DAY*(31+29+31),
	DAY*(31+29+31+30),
	DAY*(31+29+31+30+31),
	DAY*(31+29+31+30+31+30),
	DAY*(31+29+31+30+31+30+31),
	DAY*(31+29+31+30+31+30+31+31),
	DAY*(31+29+31+30+31+30+31+31+30),
	DAY*(31+29+31+30+31+30+31+31+30+31),
	DAY*(31+29+31+30+31+30+31+31+30+31+30),
];


pub fn mk_timestamp(seconds: u8, minutes: u8, hour: u8, day: u8, month: u8, year: u8) -> u32 {
    let mut timestamp: u32 = 0;

    let mut myear: u32 = year as u32;

    if year < 70 {  // mOS will not work proprely after 2069 :-)
        myear += 30;
    }

    timestamp += seconds as u32;
    timestamp += (minutes as u32) * MINUTE;
    timestamp += (hour as u32) * HOUR;
    timestamp += ((day as u32) - 1) * DAY;  // -1 because day is in progress :-)
    timestamp += MONTH[(month - 1) as usize];
    timestamp += myear * YEAR + DAY*((myear+1)/4);

    myear -= 30;
    if myear < 70 {
        myear += 2000;
    } else {
        myear += 1900;
    }
    if !is_leap_year(myear) && month > 2 {
        timestamp -= DAY;
    }

    timestamp
}


fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}
