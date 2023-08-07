use crate::{
    arguments_provider::ArgumentsProvider,
    command::{AddCommand, AddCommandArgs, Command, CommandType, ListCommand, ListCommandArgs},
    library_entry::LibraryEntry,
    list_output_handler::ListOutputHandler,
    repository::Repository,
};

pub struct CommandFactory;

impl CommandFactory {
    // pub fn new_command<'a>(
    //     command_type: CommandType,
    //     repo: &'a mut dyn Repository<LibraryEntry>,
    //     args: Box<dyn ArgumentsProvider>,
    // ) -> Box<dyn Command + 'a> {
    //     match command_type {
    //         CommandType::ADD => Self::create_add_command(repo, args),
    //         CommandType::LIST => Self::create_list_command(repo, args),
    //     }
    // }

    pub fn create_add_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        args: Box<dyn ArgumentsProvider>,
    ) -> Box<dyn Command + 'a> {
        let mut builder = AddCommand::new(repo);
        if args.argument_exists(AddCommandArgs::title.into()) {
            builder = builder.title(
                args.get_argument_string(AddCommandArgs::title.into())
                    .unwrap()
                    .to_string(),
            );
        }
        if args.argument_exists(AddCommandArgs::author.into()) {
            builder = builder.author(
                args.get_argument_string(AddCommandArgs::author.into())
                    .unwrap()
                    .to_string(),
            );
        }

        Box::new(builder.build())
    }

    pub fn create_list_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        args: Box<dyn ArgumentsProvider>,
        printer: &'a mut dyn ListOutputHandler,
    ) -> Box<dyn Command + 'a> {
        let mut builder = ListCommand::new(repo, printer);
        if args.argument_exists(ListCommandArgs::author.into()) {
            builder = builder.by_author(
                args.get_argument_string(ListCommandArgs::author.into())
                    .unwrap()
                    .to_string(),
            )
        }
        Box::new(builder.build())
    }
}
