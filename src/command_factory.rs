use crate::{
    command::{AddCommand, Command, CommandType, ListCommand},
    library_entry::LibraryEntry,
    repository::Repository,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn new_command<'a>(
        command_type: CommandType,
        repo: &'a mut dyn Repository<LibraryEntry>,
        args: &'a clap::ArgMatches,
    ) -> Box<dyn Command + 'a> {
        match command_type {
            CommandType::ADD => Box::new(AddCommand::new(repo, args)),
            CommandType::LIST => Box::new(ListCommand::new(repo, args)),
        }
    }
}
