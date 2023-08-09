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

        //TODO: alex
        //currently in command factory i am parsing through the args manually
        //add command would also have to parse these manually, so 3 places where the data is being parsed
        //these args were also parsed manually when creating the object
        //maybe an alternative would be to find a way to iterate through all the elements in the map while also storing the type

        if args.argument_exists(AddCommandArgs::title.into()) {
            builder.title(
                args.get_argument_string(AddCommandArgs::title.into())
                    .unwrap()
                    .to_string(),
            );
        }
        if args.argument_exists(AddCommandArgs::author.into()) {
            builder.author(
                args.get_argument_string(AddCommandArgs::author.into())
                    .unwrap()
                    .to_string(),
            );
        }
        if args.argument_exists(AddCommandArgs::edition.into()) {
            builder.edition(
                args.get_argument_string(AddCommandArgs::edition.into())
                    .unwrap()
                    .to_string(),
            );
        }
        if args.argument_exists(ListCommandArgs::language.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::language.into())
                .unwrap()
                .to_string();
            if let Some(val) = Language::from_string(&arg_str) {
                builder.language(val);
            }
        }
        if args.argument_exists(ListCommandArgs::read.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::read.into())
                .unwrap()
                .to_string();
            let arg_bool: Result<bool, _> = arg_str.parse();
            if arg_bool.is_ok() {
                builder.read(arg_bool.ok().unwrap());
            }
        }
        if args.argument_exists(ListCommandArgs::genre.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::genre.into())
                .unwrap()
                .to_string();
            if let Some(val) = Genre::from_string(&arg_str) {
                builder.genre(val);
            }
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
            builder.by_author(
                args.get_argument_string(ListCommandArgs::author.into())
                    .unwrap()
                    .to_string(),
            );
        }

        if args.argument_exists(ListCommandArgs::genre.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::genre.into())
                .unwrap()
                .to_string();
            if let Some(val) = Genre::from_string(&arg_str) {
                builder.by_genre(val);
            }
        }

        if args.argument_exists(ListCommandArgs::edition.into()) {
            builder.by_edition(
                args.get_argument_string(ListCommandArgs::edition.into())
                    .unwrap()
                    .to_string(),
            );
        }

        if args.argument_exists(ListCommandArgs::language.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::language.into())
                .unwrap()
                .to_string();
            if let Some(val) = Language::from_string(&arg_str) {
                builder.by_language(val);
            }
        }

        if args.argument_exists(ListCommandArgs::read.into()) {
            let arg_str = args
                .get_argument_string(ListCommandArgs::read.into())
                .unwrap()
                .to_string();
            let arg_bool: Result<bool, _> = arg_str.parse();
            if arg_bool.is_ok() {
                builder.by_read(arg_bool.ok().unwrap());
            }
        }

        Box::new(builder.build())
    }
}
