use colored::Colorize;

pub struct DoomsdayAlgorithmResultDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

pub struct DoomsdayAlgorithmResultVars {
    pub century: i32,
    pub decade: i32,
    pub month_doomsdays: i32,
    pub century_code: i32,
    pub day_to_month_doomsday_offset: i32,
    pub division_decade_by_twelve: i32,
    pub remainder_decade_by_twelve: i32,
    pub division_remainder_by_four: i32,
}

pub struct DoomsdayAlgorithmResult {
    pub date: DoomsdayAlgorithmResultDate,
    pub vars: DoomsdayAlgorithmResultVars,
    pub weekday: i32,
}

impl DoomsdayAlgorithmResult {
    pub fn display(&self) {
        println!("{}", "Date values".purple());
        println!("Year: {}", self.date.year);
        println!("Month: {}", self.date.month);
        println!("Day: {}", self.date.day);

        println!("{}", "Helper variables".purple());
        println!("Century: {}", self.vars.century);
        println!("Decade: {}", self.vars.decade);
        println!("Month doomsday: {}", self.vars.month_doomsdays);
        println!("Century code: {}", self.vars.century_code.to_string().blue().bold());
        println!(
            "Day to month doomsday offset: {}",
            self.vars.day_to_month_doomsday_offset.to_string().blue().bold()
        );
        println!(
            "Division decade by twelve: {}",
            self.vars.division_decade_by_twelve.to_string().blue().bold()
        );
        println!(
            "Remainder decade by twelve: {}",
            self.vars.remainder_decade_by_twelve.to_string().blue().bold()
        );
        println!(
            "Division remainder by four: {}",
            self.vars.division_remainder_by_four.to_string().blue().bold()
        );

        println!("Weekday: {}\n", self.weekday.to_string().green().bold());
    }
}
