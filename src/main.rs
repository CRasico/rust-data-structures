use std::io::{stdin, stdout, Write};
use std::str::FromStr;

fn main() {
    let stdin = stdin();
    let input = &mut String::new();

    loop {
        input.clear();
        let result = stdin.read_line(input);

        if result.is_err() {
            panic!("an error occured reading from the standard input")
        }

        // parse command option
        let result = MainCommand::from_str(input);
        if result.is_err() {
            panic!("failed to process the main command");
        }

        let command = result.unwrap();
        match command {
            MainCommand::Exit => return,
            MainCommand::Help(command) => command.process(),
            MainCommand::LinkedList(command) => command.process(),
            MainCommand::Unknown(command) => command.process(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum MainCommand {
    Unknown(UnknownCommand),
    Exit,
    Help(HelpCommand),
    LinkedList(LinkedListCommand),
}

impl FromStr for MainCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<MainCommand, Self::Err> {
        match input.trim() {
            "--exit" => Ok(MainCommand::Exit),
            "-e" => Ok(MainCommand::Exit),
            "--help" | "-h" => Ok(MainCommand::Help(HelpCommand {})),
            "--linkedList" | "-l" => Ok(MainCommand::LinkedList(LinkedListCommand {})),
            _ => Ok(MainCommand::Unknown(UnknownCommand {})),
        }
    }
}

trait ProcessableCommand {
    fn process(&self);
}

#[derive(Debug, PartialEq)]
struct UnknownCommand {}

impl ProcessableCommand for UnknownCommand {
    fn process(&self) {
        println!(
            r#"Unknown Command found, please see the help menu for more information.
    -h or --help for help menu
    -e or --exit to exit the program"#
        );
        stdout().flush().unwrap();
    }
}

#[derive(Debug, PartialEq)]
struct HelpCommand {}

impl ProcessableCommand for HelpCommand {
    fn process(&self) {
        // Print menu out
        println!("");
        stdout().flush().unwrap();
    }
}

#[derive(Debug, PartialEq)]
struct LinkedListCommand {}

impl ProcessableCommand for LinkedListCommand {
    fn process(&self) {
        print!("Give user ability to play with a linked list");
        stdout().flush().unwrap();
    }
}
