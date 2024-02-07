use maplit::hashmap;
use std::collections::HashMap;

pub struct Config {
    month_doomsdays: Vec<u32>,
    century_codes: HashMap<u32, u32>,
}
impl Config {
    pub fn new() -> Self {
        return Config {
            month_doomsdays: Vec::from([3, 28, 14, 4, 9, 6, 11, 8, 5, 10, 7, 12]),
            century_codes: hashmap! {
                1700 => 0,
                1800 => 5,
                1900 => 3,
                2000 => 2,
            },
        };
    }

    pub fn get_default_doomsday_for_month(&self, month: u32) -> u32 {
        return self
            .month_doomsdays
            .get((month - 1) as usize)
            .expect("Invalid month provided!")
            .to_owned();
    }

    pub fn get_century_code(&self, century: u32) -> u32 {
        return self
            .century_codes
            .get(&century)
            .expect("Invalid century provided!")
            .to_owned();
    }
}
