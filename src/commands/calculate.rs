use std::error::Error;

use crate::doomsday::DoomsdayAlgorithm;
use colored::Colorize;

pub struct Calculate {
    date_str: String,
}

impl Calculate {
    pub fn new(date_str: String) -> Self {
        return Calculate { date_str };
    }

    pub fn run(&mut self) -> Result<u8, Box<dyn Error>> {
        let result = DoomsdayAlgorithm::execute(&self.date_str).expect("Failed to calculate date!");

        println!("Result: {}", result.weekday.to_string().green().bold());

        return Ok(0);
    }
}
