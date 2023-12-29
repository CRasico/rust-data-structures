use std::io::{stdin, stdout, Write};

use crate::{
    core::graph::{GraphNode, RotationalGraph},
    view::processable_command::ProcessableCommand,
};

#[derive(Debug, PartialEq)]
pub struct GraphCommand {}

impl ProcessableCommand for GraphCommand {
    fn process(&self) {
        let mut rotational_graph: RotationalGraph<String> = RotationalGraph::new();

        GraphCommand::print_help_menu();

        loop {
            let operation_result = Self::fetch_operation();

            match operation_result {
                Err(message) => {
                    println!("{}", message);
                    stdout().flush().unwrap();
                    continue;
                }
                Ok(operation) => match operation {
                    GraphOperation::Unknown => continue,
                    GraphOperation::Help => GraphCommand::print_help_menu(),
                    GraphOperation::Add(value) => {
                        let node = GraphNode::new(value);
                        rotational_graph.add(node);
                    }
                    GraphOperation::Remove => {
                        rotational_graph.remove();
                    }
                    GraphOperation::RotateCurrent => {
                        rotational_graph.rotate_current();
                    }
                    GraphOperation::PrintFromCurrent => {
                        println!("{:?}", rotational_graph.get_current());
                        stdout().flush().unwrap();
                    }
                    GraphOperation::Exit => {
                        return;
                    }
                },
            }
        }
    }
}

impl GraphCommand {
    fn print_help_menu() {}

    fn fetch_operation() -> Result<GraphOperation, String> {
        let stdin = stdin();
        let input = &mut String::new();

        println!("\nWhat operation would you like to take?\n");
        stdout().flush().unwrap();

        let result = stdin.read_line(input);
        if result.is_err() {
            return Err(String::from(
                "an error occured reading from the standard input",
            ));
        }

        //TODO: Would be interesting to make this more reusable, revisit in the future
        let result = Self::parse_operation(input);
        if result.is_err() {
            return Err(String::from("failed to process the main command"));
        }

        match result {
            Ok(operation) => Ok(operation),
            Err(parse_error) => Err(String::from(parse_error)),
        }
    }

    fn parse_operation(input: &str) -> Result<GraphOperation, &str> {
        let trimmed = input.trim();
        let collection: Vec<&str> = trimmed.split(' ').collect();
        if collection.len() == 0 {
            return Err("You must input at least one command");
        }
        match collection[0] {
            "--help" | "-h" => Ok(GraphOperation::Help),
            "--print-from-current" | "-pfc" => Ok(GraphOperation::PrintFromCurrent),
            "--exit" | "-e" => Ok(GraphOperation::Exit),
            "--add" | "-a" => {
                if collection.len() < 2 {
                    return Err("When adding a node you must provide a name for the node");
                }
                return Ok(GraphOperation::Add(String::from(collection[1])));
            }
            "--rotate-current" | "-rc" => Ok(GraphOperation::RotateCurrent),
            "--remove-current" | "-rm" => Ok(GraphOperation::Remove),
            _ => Ok(GraphOperation::Unknown),
        }
    }
}

enum GraphOperation {
    Unknown,
    Help,
    Exit,
    PrintFromCurrent,
    Add(String),
    RotateCurrent,
    Remove,
}
