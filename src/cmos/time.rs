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

pub fn mk_timestamp(seconds: u8, minutes: u8, hour: u8, day: u8, month: u8, year: u8) -> u32 {
    let mut timestamp: u32 = 0;
    let mut myear: u32 = year as u32;

    if year < 70 {
        // mOS will not work proprely after 2069 :-)
        myear += 30;
    } else {
        myear -= 70;
    }

    timestamp += seconds as u32;
    timestamp += (minutes as u32) * MINUTE;
    timestamp += (hour as u32) * HOUR;
    timestamp += ((day as u32) - 1) * DAY; // -1 because day is in progress :-)
    timestamp += MONTH[(month - 1) as usize];
    timestamp += myear * YEAR + DAY * ((myear + 1) / 4);

    if !is_leap_year(myear + 1970) && month > 2 {
        timestamp -= DAY;
    }

    timestamp
}

fn is_leap_year(year: u32) -> bool {
    return (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mk_timestamp_zero() {
        assert_eq!(mk_timestamp(0, 0, 0, 1, 1, 70), 0)
    }

    #[test]
    fn mk_timestamp_for_leap_year_and_january() {
        assert_eq!(mk_timestamp(0, 0, 0, 01, 01, 48), 2461449600)
    }

    #[test]
    fn mk_timestamp_for_leap_year_and_march() {
        assert_eq!(mk_timestamp(0, 0, 0, 01, 03, 48), 2466633600)
    }

    #[test]
    fn mk_timestamp_after_2000() {
        assert_eq!(mk_timestamp(17, 13, 13, 06, 11, 18), 1541509997)
    }

    #[test]
    fn mk_timestamp_my() {
        assert_eq!(mk_timestamp(0, 0, 0, 25, 07, 90), 648864000)
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
