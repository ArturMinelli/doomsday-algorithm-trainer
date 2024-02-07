use chrono::NaiveDate;

struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

pub struct Config {
    date_range: DateRange
}

impl Config {
    pub fn new() -> Self {
        return Config {
            date_range: DateRange {
                start: NaiveDate::from_ymd_opt(1700, 1, 1).unwrap(),
                end: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
            }
        }
    }

    pub fn get_date_range(&self) -> (NaiveDate, NaiveDate) {
        return (self.date_range.start, self.date_range.end);
    }
}
