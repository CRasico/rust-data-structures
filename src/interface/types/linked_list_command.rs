use std::io::{stdout, Write};

use crate::interface::processable_command::ProcessableCommand;

#[derive(Debug, PartialEq)]
pub struct LinkedListCommand {}

impl ProcessableCommand for LinkedListCommand {
    fn process(&self) {
        print!("Give user ability to play with a linked list");
        stdout().flush().unwrap();
    }
}
