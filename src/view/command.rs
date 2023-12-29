use std::str::FromStr;

use super::types::{
    graph_command::GraphCommand, help_command::HelpCommand, linked_list_command::LinkedListCommand,
    unknown_command::UnknownCommand,
};

#[derive(Debug, PartialEq)]
pub enum MainCommand {
    Unknown(UnknownCommand),
    Exit,
    Help(HelpCommand),
    LinkedList(LinkedListCommand),
    Graph(GraphCommand),
}

impl FromStr for MainCommand {
    type Err = ();

    fn from_str(input: &str) -> Result<MainCommand, Self::Err> {
        match input.trim() {
            "--exit" | "-e" => Ok(MainCommand::Exit),
            "--help" | "-h" => Ok(MainCommand::Help(HelpCommand {})),
            "--linkedList" | "-ll" => Ok(MainCommand::LinkedList(LinkedListCommand {})),
            "--graph" | "g" => Ok(MainCommand::Graph(GraphCommand {})),
            _ => Ok(MainCommand::Unknown(UnknownCommand {})),
        }
    }
}
