use crate::{
    config::Config,
    doomsday::{DoomsdayAlgorithm, DoomsdayAlgorithmResult},
    helpers::RandomDateGenerator,
};
use colored::Colorize;

enum UserAction {
    Continue,
    Quit,
}

pub struct Train {}
impl Train {
    pub fn new() -> Self {
        return Train {};
    }

    pub fn run(&self) -> Result<u8, String> {
        let result = loop {
            self.clear_output();

            let date = self.get_random_date();
            println!("Date: {}", date.purple());

            let result = self.calculate_weekday(date);
            let answer = self
                .read_input()
                .parse::<i32>()
                .expect("Answer should be a number!");
            if answer == result.weekday {
                println!("{}", "Correct!".cyan());
            } else {
                println!("{}", "Incorrect!".red());
                result.display();
            }

            println!("Press 'enter' to continue or 'q' to quit.");
            let action = self.get_user_action();
            match action {
                UserAction::Continue => continue,
                UserAction::Quit => break 0,
            }
        };

        return Ok(result);
    }

    fn get_user_action(&self) -> UserAction {
        return loop {
            let action = self.read_input();

            match action.as_str().trim() {
                "" => {
                    break UserAction::Continue;
                }

                "q" => {
                    break UserAction::Quit;
                }

                _ => {
                    println!("Invalid action!");
                    continue;
                }
            }
        };
    }

    fn read_input(&self) -> String {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Cannot read user input");

        return input.trim().to_string();
    }

    fn clear_output(&self) -> () {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn get_random_date(&self) -> String {
        let (start, end) = Config::new().get_date_range();

        return RandomDateGenerator::execute(start, end);
    }

    fn calculate_weekday(&self, date_str: String) -> DoomsdayAlgorithmResult {
        return DoomsdayAlgorithm::execute(&date_str).expect("Failed to calculate weekday");
    }
}
