use chrono::{Days, NaiveDate};
use rand::Rng;

pub struct RandomDateGenerator {}
impl RandomDateGenerator {
    pub fn execute(start: NaiveDate, end: NaiveDate) -> String {
        let days_in_range = (end - start).num_days();

        let days = rand::thread_rng().gen_range(0..days_in_range);

        return start
            .checked_add_days(Days::new(days as u64))
            .unwrap()
            .format("%d-%m-%Y")
            .to_string();
    }
}
