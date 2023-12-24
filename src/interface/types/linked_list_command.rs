use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

use crate::{core::linked_list::LinkedList, interface::processable_command::ProcessableCommand};

#[derive(Debug, PartialEq)]
pub struct LinkedListCommand {}

impl ProcessableCommand for LinkedListCommand {
    fn process(&self) {
        let mut linked_list: LinkedList<i32> = LinkedList::new();

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
                        println!("Unknown command reached!");
                        stdout().flush().unwrap();
                    }
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
                        println!("{:?}", linked_list);
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
    fn fetch_operation() -> Result<LinkedListOperation, String> {
        let stdin = stdin();
        let input = &mut String::new();

        println!("\nWhat operation would you like to take?");

        println!("-ps, --push: to push an item to the list");
        println!("-pk, --peek: to peek at the top item in the list");
        println!("-pp, --pop: to pop the top item from the list");
        println!("-p, --print: to the entire list");
        println!("-e, --exit: will allow you to exit the appliation");
        stdout().flush().unwrap();

        let result = stdin.read_line(input);
        if result.is_err() {
            return Err(String::from(
                "an error occured reading from the standard input",
            ));
        }

        // parse command option
        let result = LinkedListOperation::from_str(input);
        if result.is_err() {
            return Err(String::from("failed to process the main command"));
        }

        println!("");
        stdout().flush().unwrap();

        match result {
            Ok(operation) => Ok(operation),
            Err(parse_error) => Err(parse_error.message),
        }
    }
}

enum LinkedListOperation {
    Unknown,
    IsEmpty,
    Push(i32),
    Pop,
    Peek,
    Print,
    Exit,
}

struct LinkedListOperationParseErr {
    message: String,
}

impl FromStr for LinkedListOperation {
    type Err = LinkedListOperationParseErr;

    fn from_str(input: &str) -> Result<LinkedListOperation, Self::Err> {
        match input.trim() {
            "--is-empy" | "-ie" => Ok(LinkedListOperation::IsEmpty),
            "--push" | "-ps" => {
                let input = &mut String::new();

                print!("Please enter a number: ");
                stdout().flush().unwrap();

                let result = stdin().read_line(input);
                if result.is_err() {
                    return Err(LinkedListOperationParseErr {
                        message: String::from("an error occured reading from the standard input"),
                    });
                }

                let number_result = input.trim().parse::<i32>();
                if number_result.is_err() {
                    return Err(LinkedListOperationParseErr {
                        message: String::from("The given input was not a number"),
                    });
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
