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

#[derive(IntoStaticStr, EnumIter, Display)]
pub enum ListCommandArgs {
    author,
    read,
    edition,
    genre,
    language,
}

pub struct ListCommand<'a> {
    repo: &'a dyn Repository<LibraryEntry>,
    author: Option<String>,
    genre: Option<Genre>,
    edition: Option<String>,
    language: Option<Language>,
    read: Option<bool>,
    output_handler: &'a mut dyn ListOutputHandler,
}

impl ListCommand<'_> {
    pub fn new<'a>(
        repo: &'a dyn Repository<LibraryEntry>,
        output_handler: &'a mut dyn ListOutputHandler,
    ) -> ListCommandBuilder<'a> {
        ListCommandBuilder {
            repo,
            author_: None,
            genre_: None,
            edition_: None,
            language_: None,
            read_: None,
            output_handler_: output_handler,
        }
    }
}

pub struct ListCommandBuilder<'a> {
    author_: Option<String>,
    genre_: Option<Genre>,
    edition_: Option<String>,
    language_: Option<Language>,
    read_: Option<bool>,
    repo: &'a dyn Repository<LibraryEntry>,
    output_handler_: &'a mut dyn ListOutputHandler,
}

impl<'a> ListCommandBuilder<'a> {
    pub fn build(self) -> ListCommand<'a> {
        ListCommand {
            repo: self.repo,
            author: self.author_,
            genre: self.genre_,
            edition: self.edition_,
            language: self.language_,
            read: self.read_,
            output_handler: self.output_handler_,
        }
    }

    pub fn by_genre(&mut self, genre: Genre) -> &Self {
        self.genre_ = Some(genre);
        self
    }

    pub fn by_author(&mut self, author: String) -> &Self {
        self.author_ = Some(author);
        self
    }

    pub fn by_edition(&mut self, edition: String) -> &Self {
        self.edition_ = Some(edition);
        self
    }

    pub fn by_language(&mut self, language: Language) -> &Self {
        self.language_ = Some(language);
        self
    }

    pub fn by_read(&mut self, read: bool) -> &Self {
        self.read_ = Some(read);
        self
    }
}

impl Command for ListCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing list command");
        println!("Repo size is {}", self.repo.get_all().len());
        let mut result: Vec<LibraryEntry> = self.repo.get_all().clone();
        //Note to self:
        //The syntax differs from the command_api calls because there the Option<String> is consumed, but here we cannot consume a self which is a &mut
        if let Some(val) = self.author.as_mut() {
            result.retain(|x| x.book.cover_info.author == *val);
        }

        if let Some(val) = &self.genre {
            result.retain(|x| x.book.genre == *val);
        }

        if let Some(val) = &self.language {
            result.retain(|x| x.book.language == *val);
        }

        if let Some(val) = &self.edition {
            result.retain(|x| x.book.cover_info.edition == *val);
        }

        if let Some(val) = &self.read {
            result.retain(|x| x.metadata.read == *val);
        }

        self.output_handler.handle_list_output(&result);
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undoing list command");
        true
    }
}
