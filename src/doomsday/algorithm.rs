use crate::doomsday::Config;
use crate::doomsday::{
    DoomsdayAlgorithmResult, DoomsdayAlgorithmResultDate, DoomsdayAlgorithmResultVars,
};
use chrono::{Datelike, NaiveDate};

pub struct DoomsdayAlgorithm {}
impl DoomsdayAlgorithm {
    pub fn execute(date_str: &str) -> Result<DoomsdayAlgorithmResult, String> {
        let date = Self::parse_date(date_str);

        // Decomposing the date
        let year = date.year();
        let month = date.month();
        let day = date.day();

        // Helper values
        let config = Config::new();
        let century = (year / 100) * 100;
        let decade = year - century;
        let leap_year_exception = Self::check_leap_year_exception(month, year, decade);
        let century_code = config.get_century_code(century as u32);
        let default_month_doomsday = config.get_default_doomsday_for_month(month);
        let month_doomsday = if leap_year_exception {
            default_month_doomsday + 1
        } else {
            default_month_doomsday
        };

        // The algorithm itself
        let day_to_month_doomsday_offset = day as i32 - month_doomsday as i32;
        let division_decade_by_twelve = (decade / 12) as i32;
        let remainder_decade_by_twelve = (decade % 12) as i32;
        let division_remainder_by_four = (remainder_decade_by_twelve / 4) as i32;

        let mut weekday = (day_to_month_doomsday_offset
            + century_code as i32
            + division_decade_by_twelve
            + remainder_decade_by_twelve
            + division_remainder_by_four)
            % 7;

        weekday = if weekday >= 0 { weekday } else { weekday + 7 };

        return Ok(DoomsdayAlgorithmResult {
            date: DoomsdayAlgorithmResultDate {
                year,
                month: month as i32,
                day: day as i32,
            },
            vars: DoomsdayAlgorithmResultVars {
                century,
                decade,
                century_code: century_code as i32,
                month_doomsdays: month_doomsday as i32,
                day_to_month_doomsday_offset,
                division_decade_by_twelve,
                remainder_decade_by_twelve,
                division_remainder_by_four,
            },
            weekday,
        });
    }

    fn parse_date(date_str: &str) -> NaiveDate {
        return NaiveDate::parse_from_str(date_str, "%d-%m-%Y").expect("Invalid date string!");
    }

    fn check_leap_year_exception(month: u32, year: i32, decade: i32) -> bool {
        if year % 100 == 0 && year % 400 != 0 {
            return false;
        };

        return decade % 4 == 0 && (month == 1 || month == 2);
    }
}
