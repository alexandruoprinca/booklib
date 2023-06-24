use crate::{
    command::{AddCommand, Command, CommandType, ListCommand},
    library_entry::LibraryEntry,
    repository::Repository,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn new_command<'a>(
        command_type: CommandType,
        mut repo: &'a dyn Repository<LibraryEntry>,
        args: &clap::ArgMatches,
    ) -> Box<dyn Command + 'a> {
        match command_type {
            CommandType::ADD => Box::new(AddCommand::new(String::from("test"), repo)),
            CommandType::LIST => Box::new(ListCommand::new(repo)),
        }
    }
}
