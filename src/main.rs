mod commands;
mod config;
mod doomsday;
mod helpers;

use clap::{command, Arg, Command};

use commands::{Calculate, Train};

fn main() {
    let command = command!()
        .subcommand(
            Command::new("calculate")
                .arg(
                    Arg::new("date")
                        .short('d')
                        .long("date")
                        .required(true)
                        .help("A date in the format YYYY-mm-dd.")
                )
        )
        .subcommand(
            Command::new("train")
        ).get_matches();

    let subcommand = command
        .subcommand()
        .expect("Please provide a valid command!");

    match subcommand {
        ("calculate", args) => {
            let date = args.get_one::<String>("date").unwrap().to_owned();

            let _result = Calculate::new(date).run().unwrap();
        }

        ("train", _args) => {
            let _result = Train::new().run().unwrap();
        }

        _ => {
            panic!("Please provide a valid command!");
        }
    }
}
