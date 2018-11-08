use core::fmt;

const MINUTE: u32 = 60;
const HOUR: u32 = MINUTE * 60;
const DAY: u32 = HOUR * 24;
const YEAR: u32 = DAY * 365;
static MONTH: [u32; 12] = [
    0,
    DAY * (31),
    DAY * (31 + 29),
    DAY * (31 + 29 + 31),
    DAY * (31 + 29 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31),
    DAY * (31 + 29 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30),
];

pub struct DateTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hour: u8,
    pub month: u8,
    pub day: u8,
    pub year: u8,
}

impl DateTime {
    pub fn timestamp(&self) -> u32 {
        let mut timestamp: u32 = 0;
        let mut myear: u32 = self.year as u32;

        if self.year < 70 {
            // mOS will not work proprely after 2069 :-)
            myear += 30;
        } else {
            myear -= 70;
        }

        timestamp += self.seconds as u32;
        timestamp += (self.minutes as u32) * MINUTE;
        timestamp += (self.hour as u32) * HOUR;
        timestamp += ((self.day as u32) - 1) * DAY; // -1 because day is in progress :-)
        timestamp += MONTH[(self.month - 1) as usize];
        timestamp += myear * YEAR + DAY * ((myear + 1) / 4);

        if !is_leap_year(myear + 1970) && self.month > 2 {
            timestamp -= DAY;
        }

        timestamp
    }

    pub fn full_year(&self) -> u16 {
        let mut year: u16 = self.year as u16;
        if self.year < 70 {
            year += 2000;
        } else {
            year += 1970;
        };
        year
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}-{:02}-{:02}T{:02}:{:02}:{:02}",
            self.full_year(),
            self.month,
            self.day,
            self.hour,
            self.minutes,
            self.seconds,
        )
    }
}

fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mk_timestamp_zero() {
        let dt = DateTime {
            seconds: 0,
            minutes: 0,
            hour: 0,
            day: 1,
            month: 1,
            year: 70,
        };
        assert_eq!(dt.timestamp(), 0)
    }

    #[test]
    fn mk_timestamp_for_leap_year_and_january() {
        let dt = DateTime {
            seconds: 0,
            minutes: 0,
            hour: 0,
            day: 1,
            month: 1,
            year: 48,
        };
        assert_eq!(dt.timestamp(), 2461449600)
    }

    #[test]
    fn mk_timestamp_for_leap_year_and_march() {
        let dt = DateTime {
            seconds: 0,
            minutes: 0,
            hour: 0,
            day: 1,
            month: 3,
            year: 48,
        };
        assert_eq!(dt.timestamp(), 2466633600)
    }

    #[test]
    fn mk_timestamp_after_2000() {
        let dt = DateTime {
            seconds: 17,
            minutes: 13,
            hour: 13,
            day: 06,
            month: 11,
            year: 18,
        };
        assert_eq!(dt.timestamp(), 1541509997)
    }

    #[test]
    fn mk_timestamp_my() {
        let dt = DateTime {
            seconds: 00,
            minutes: 00,
            hour: 00,
            day: 25,
            month: 07,
            year: 90,
        };
        assert_eq!(dt.timestamp(), 648864000)
    }

    #[test]
    fn test_is_leap_year() {
        let leap_years: [u32; 37] = [
            1904, 1908, 1912, 1916, 1920, 1924, 1928, 1932, 1936, 1940, 1944, 1948, 1952, 1956,
            1960, 1964, 1968, 1972, 1976, 1980, 1984, 1988, 1992, 1996, 2000, 2004, 2008, 2012,
            2016, 2020, 2024, 2028, 2032, 2036, 2040, 2044, 2048,
        ];
        for year in leap_years.iter() {
            assert_eq!(is_leap_year(*year), true);
        }
    }

    #[test]
    fn test_is_not_leap_year() {
        let not_leap_years: [u32; 26] = [
            1905, 1909, 1911, 1917, 1921, 1922, 1923, 1925, 1926, 1927, 1929, 1930, 1931, 2007,
            2009, 2010, 2011, 2013, 2014, 2015, 2017, 2018, 2019, 2021, 2022, 2023,
        ];
        for year in not_leap_years.iter() {
            assert_eq!(is_leap_year(*year), false);
        }
    }
}
