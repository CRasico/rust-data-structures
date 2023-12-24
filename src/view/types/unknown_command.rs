use std::io::{stdout, Write};

use crate::view::processable_command::ProcessableCommand;

#[derive(Debug, PartialEq)]
pub struct UnknownCommand {}

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
