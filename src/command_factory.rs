use crate::{
    command::{
        AddCommand, AddCommandOptions, Command, CommandType, ListCommand, ListCommandOptions,
    },
    library_entry::{Genre, Language, LibraryEntry},
    list_output_handler::ListOutputHandler,
    repository::Repository,
};

pub struct CommandFactory;

impl CommandFactory {
    pub fn create_add_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        options: Vec<AddCommandOptions>,
    ) -> Box<dyn Command + 'a> {
        let mut builder = AddCommand::new(repo);
        for option in options {
            builder.add_option(option);
        }

        Box::new(builder.build())
    }

    pub fn create_list_command<'a>(
        repo: &'a mut dyn Repository<LibraryEntry>,
        options: Vec<ListCommandOptions>,
        printer: &'a mut dyn ListOutputHandler,
    ) -> Box<dyn Command + 'a> {
        let mut builder = ListCommand::new(repo, printer);
        for option in options {
            builder.add_option(option);
        }
        Box::new(builder.build())
    }
}
