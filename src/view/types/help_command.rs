use std::io::{stdout, Write};

use crate::view::processable_command::ProcessableCommand;

#[derive(Debug, PartialEq)]
pub struct HelpCommand {}

impl ProcessableCommand for HelpCommand {
    fn process(&self) {
        // Print menu out
        println!("** Welcome to Rust Data Structures **");
        println!("   This console application with give you an overview of various data structures and their usage in rust\n");
        println!("Available Commands");
        println!("-e, --exit: will allow you to exit the appliation");
        println!("-h, --help: will open this exact helper menu");
        println!("-ll, --linkedList: will allow you to edit and update a linked list");
        println!("-g, --graph: will allow you to edit and update a rotational graph");
        stdout().flush().unwrap();
    }
}
