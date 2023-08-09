use crate::{
    arguments_provider::ArgumentsProvider,
    command::{AddCommand, AddCommandArgs, Command, CommandType, ListCommand, ListCommandArgs},
    library_entry::{Genre, Language, LibraryEntry},
    list_output_handler::ListOutputHandler,
    repository::Repository,
};

pub struct CommandFactory;

impl CommandFactory {
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

        if args.argument_exists(ListCommandArgs::genre.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::genre.into())
                .unwrap()
                .to_string();
            if let Some(val) = Genre::from_string(&arg_str) {
                builder = builder.by_genre(val);
            }
        }

        if args.argument_exists(ListCommandArgs::edition.into()) {
            builder = builder.by_edition(
                args.get_argument_string(ListCommandArgs::edition.into())
                    .unwrap()
                    .to_string(),
            )
        }

        if args.argument_exists(ListCommandArgs::language.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::language.into())
                .unwrap()
                .to_string();
            if let Some(val) = Language::from_string(&arg_str) {
                builder = builder.by_language(val);
            }
        }

        if args.argument_exists(ListCommandArgs::read.into()) {
            let arg_str = args.get_argument_string(ListCommandArgs::read.into()).unwrap().to_string();
            let arg_bool: Result<bool, _> = arg_str.parse();
            if arg_bool.is_ok() {
                builder = builder.by_read(arg_bool.ok().unwrap());
            }
        }

        Box::new(builder.build())
    }
}
