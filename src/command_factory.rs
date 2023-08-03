use crate::{
    command::{AddCommand, AddCommandArgs, Command, CommandType, ListCommand, ListCommandArgs},
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
            CommandType::LIST => Self::create_list_command(repo, args),
        }
    }

    pub fn create_add_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        args: &'a clap::ArgMatches,
    ) -> Box<dyn Command + 'a> {
        let mut builder = AddCommand::new(repo);
        if args.contains_id(AddCommandArgs::title.into()) {
            builder = builder.title(
                args.get_one::<String>(AddCommandArgs::title.into())
                    .unwrap()
                    .to_string(),
            );
        }
        if args.contains_id(AddCommandArgs::author.into()) {
            builder = builder.author(
                args.get_one::<String>(AddCommandArgs::author.into())
                    .unwrap()
                    .to_string(),
            );
        }

        Box::new(builder.build())
    }

    pub fn create_list_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        args: &'a clap::ArgMatches,
    ) -> Box<dyn Command + 'a> {
        let mut builder = ListCommand::new(repo);
        if args.contains_id(ListCommandArgs::author.into()){
            builder = builder.by_author(
                args.get_one::<String>(ListCommandArgs::author.into()).unwrap().to_string()
            )
        }
        Box::new(builder.build())
    }
}
