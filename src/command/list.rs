use std::error::Error;

use rocket::http::RawStr;
use rocket::request::FromParam;
use rocket::response::Debug;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use super::Command;
use crate::library_entry::{Genre, Language};
use crate::list_output_handler::{ConsoleOutputHandler, ListOutputHandler};
use crate::{library_entry::LibraryEntry, repository::Repository};

pub enum ListCommandOptions {
    author(String),
    genre(Genre),
    edition(String),
    language(Language),
    read(bool),
}

pub struct ListCommand<'a> {
    repo: &'a dyn Repository<LibraryEntry>,
    options: Vec<ListCommandOptions>,
    output_handler: &'a mut dyn ListOutputHandler,
}

impl ListCommand<'_> {
    pub fn new<'a>(
        repo: &'a dyn Repository<LibraryEntry>,
        output_handler: &'a mut dyn ListOutputHandler,
    ) -> ListCommandBuilder<'a> {
        ListCommandBuilder {
            repo,
            command_options: Vec::default(),
            output_handler_: output_handler,
        }
    }

    pub fn filter_by(
        mut entries: Vec<LibraryEntry>,
        option: &ListCommandOptions,
    ) -> Vec<LibraryEntry> {
        entries.retain(|x| match option {
            ListCommandOptions::author(author_val) => x.book.cover_info.author == *author_val,
            ListCommandOptions::genre(genre_val) => x.book.genre == *genre_val,
            ListCommandOptions::edition(edition_val) => x.book.cover_info.edition == *edition_val,
            ListCommandOptions::language(language_val) => x.book.language == *language_val,
            ListCommandOptions::read(read_val) => x.metadata.read == *read_val,
        });
        entries
    }
}

pub struct ListCommandBuilder<'a> {
    command_options: Vec<ListCommandOptions>,
    repo: &'a dyn Repository<LibraryEntry>,
    output_handler_: &'a mut dyn ListOutputHandler,
}

impl<'a> ListCommandBuilder<'a> {
    pub fn build(self) -> ListCommand<'a> {
        ListCommand {
            repo: self.repo,
            options: self.command_options,
            output_handler: self.output_handler_,
        }
    }

    pub fn add_option(&mut self, option: ListCommandOptions) -> &Self {
        self.command_options.push(option);
        self
    }
}

impl Command for ListCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing list command");
        println!("Repo size is {}", self.repo.get_all().len());
        let mut result: Vec<LibraryEntry> = self.repo.get_all().clone();
        for option in &self.options {
            result = ListCommand::filter_by(result, option);
        }

        self.output_handler.handle_list_output(&result);
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undoing list command");
        true
    }
}
