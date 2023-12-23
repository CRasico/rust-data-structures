use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use crate::interface::command::MainCommand;

pub struct FetchInputCommand {}

impl FetchInputCommand {
    pub fn next() -> Result<MainCommand, String> {
        let stdin = stdin();
        let input = &mut String::new();

        print!("\nWhat would you like to do: ");
        stdout().flush().unwrap();

        let result = stdin.read_line(input);
        if result.is_err() {
            return Err(String::from(
                "an error occured reading from the standard input",
            ));
        }

        // parse command option
        let result = MainCommand::from_str(input);
        if result.is_err() {
            return Err(String::from("failed to process the main command"));
        }

        println!("");
        stdout().flush().unwrap();

        return Ok(result.unwrap());
    }
}
