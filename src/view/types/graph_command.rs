use crate::{core::graph::GraphNode, view::processable_command::ProcessableCommand};

pub struct GraphCommand {}

impl ProcessableCommand for GraphCommand {
    fn process(&self) {}
}

impl GraphCommand {
    fn fetch_operation() {}

    fn parse_operation(input: &str) -> Result<GraphOperation, &str> {
        let trimmed = input.trim();
        let collection: Vec<&str> = trimmed.split(' ').collect();
        if collection.len() == 0 {
            return Err("You must input at least one command");
        }
        match collection[0] {
            "--help" | "-h" => Ok(GraphOperation::Help),
            "--print-from-current" | "-pfc" => Ok(GraphOperation::PrintFromCurrent),
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
    PrintFromCurrent,
    Add(String),
    RotateCurrent,
    Remove,
}
