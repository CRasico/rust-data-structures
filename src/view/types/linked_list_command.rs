use std::io::{stdin, stdout, Write};

use crate::{core::linked_list::LinkedList, view::processable_command::ProcessableCommand};

#[derive(Debug, PartialEq)]
pub struct LinkedListCommand {}

impl ProcessableCommand for LinkedListCommand {
    fn process(&self) {
        let mut linked_list: LinkedList<i32> = LinkedList::new();

        LinkedListCommand::print_help_menu();

        loop {
            let operation_result = Self::fetch_operation();

            match operation_result {
                Err(message) => {
                    println!("{}", message);
                    stdout().flush().unwrap();
                    continue;
                }
                Ok(operation) => match operation {
                    LinkedListOperation::Unknown => {
                        println!("Input command unknown, please try again.");
                        stdout().flush().unwrap();
                    }
                    LinkedListOperation::Help => LinkedListCommand::print_help_menu(),
                    LinkedListOperation::IsEmpty => {
                        println!("Is list empty: {}", linked_list.empty());
                        stdout().flush().unwrap();
                    }
                    LinkedListOperation::Push(item) => {
                        linked_list.push(item);
                    }
                    LinkedListOperation::Peek => {
                        let value = linked_list.peek();
                        match value {
                            Some(number) => println!("The value is: {}", number),
                            None => println!("There is no top value"),
                        }
                        stdout().flush().unwrap();
                    }
                    LinkedListOperation::Pop => {
                        linked_list.pop();
                    }
                    LinkedListOperation::Print => {
                        println!("{}", linked_list.to_string());
                    }
                    LinkedListOperation::Exit => {
                        return;
                    }
                },
            }
        }
    }
}

impl LinkedListCommand {
    fn print_help_menu() {
        println!("-ps, --push: to push an item to the list");
        println!("-pk, --peek: to peek at the top item in the list");
        println!("-pp, --pop: to pop the top item from the list");
        println!("-p, --print: to the entire list");
        println!("-e, --exit: will allow you to exit the appliation");
        stdout().flush().unwrap();
    }
    fn fetch_operation() -> Result<LinkedListOperation, String> {
        let stdin = stdin();
        let input = &mut String::new();

        println!("\nWhat operation would you like to take?");
        stdout().flush().unwrap();

        let result = stdin.read_line(input);
        if result.is_err() {
            return Err(String::from(
                "an error occured reading from the standard input",
            ));
        }

        let result = Self::parse_operation(input);
        if result.is_err() {
            return Err(String::from("failed to process the main command"));
        }

        println!("");
        stdout().flush().unwrap();

        match result {
            Ok(operation) => Ok(operation),
            Err(parse_error) => Err(String::from(parse_error)),
        }
    }

    fn parse_operation(input: &str) -> Result<LinkedListOperation, &str> {
        let collection: Vec<&str> = input.trim().split(' ').collect();
        if collection.len() == 0 {
            return Err("You must input at least one commmand");
        }
        match collection[0] {
            "--help" | "-h" => Ok(LinkedListOperation::Help),
            "--is-empy" | "-ie" => Ok(LinkedListOperation::IsEmpty),
            "--push" | "-ps" => {
                if collection.len() < 2 {
                    return Err(
                        "When pushing a node, you must provide a number value for the node",
                    );
                }

                let number_result = collection[1].parse::<i32>();
                if number_result.is_err() {
                    return Err("The node value was not an integer");
                }

                return Ok(LinkedListOperation::Push(number_result.unwrap()));
            }
            "--pop" | "-pp" => Ok(LinkedListOperation::Pop),
            "--peek" | "-pk" => Ok(LinkedListOperation::Peek),
            "--print" | "-p" => Ok(LinkedListOperation::Print),
            "--exit" | "-e" => Ok(LinkedListOperation::Exit),
            _ => Ok(LinkedListOperation::Unknown),
        }
    }
}

enum LinkedListOperation {
    Unknown,
    Help,
    IsEmpty,
    Push(i32),
    Pop,
    Peek,
    Print,
    Exit,
}
