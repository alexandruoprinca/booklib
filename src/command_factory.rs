use crate::{
    command::{AddCommand, Command, CommandType, ListCommand, AddCommandArgs},
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
            CommandType::ADD => Self::create_add_command(repo, args),
            CommandType::LIST => Box::new(ListCommand::new(repo, args)),
        }
    }

    pub fn create_add_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        args: &'a clap::ArgMatches,
    ) -> Box<dyn Command + 'a> {
        let mut builder = AddCommand::new(repo);
        if args.contains_id(AddCommandArgs::title.into()) {
            builder = builder.title(args.get_one::<String>(AddCommandArgs::title.into()).unwrap().to_string());
        }
        if args.contains_id(AddCommandArgs::author.into()) {
            builder = builder.author(args.get_one::<String>(AddCommandArgs::author.into()).unwrap().to_string());
        }

        Box::new(builder.build())
    }
}
