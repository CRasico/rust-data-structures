use rust_data_structures::interface::command::MainCommand;
use rust_data_structures::interface::fetch_input_command::FetchInputCommand;
use rust_data_structures::interface::processable_command::ProcessableCommand;
use rust_data_structures::interface::types::help_command::HelpCommand;

fn main() {
    let introduction = HelpCommand {};
    introduction.process();

    loop {
        let result = FetchInputCommand::next();
        match result {
            Ok(command) => match command {
                MainCommand::Exit => return,
                MainCommand::Help(command) => command.process(),
                MainCommand::LinkedList(command) => command.process(),
                MainCommand::Unknown(command) => command.process(),
            },
            Err(message) => panic!("{}", message),
        }
    }
}
