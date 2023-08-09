extern crate strum;
use strum_macros::IntoStaticStr;

use crate::{
    library_entry::{Genre, Language, LibraryEntry},
    repository::Repository,
};

use super::Command;

#[derive(IntoStaticStr)]
pub enum AddCommandArgs {
    title,
    author,
    genre,
    edition,
    language,
    read,
}

pub struct AddCommand<'a> {
    repo: &'a mut dyn Repository<LibraryEntry>,
    title: Option<String>,
    author: Option<String>,
    genre: Option<Genre>,
    edition: Option<String>,
    language: Option<Language>,
    read: Option<bool>,
}

impl AddCommand<'_> {
    pub fn new<'a>(repo: &'a mut dyn Repository<LibraryEntry>) -> AddCommandBuilder<'a> {
        AddCommandBuilder {
            repo,
            title_: None,
            author_: None,
            genre_: None,
            edition_: None,
            language_: None,
            read_: None,
        }
    }
}

pub struct AddCommandBuilder<'a> {
    title_: Option<String>,
    author_: Option<String>,
    genre_: Option<Genre>,
    edition_: Option<String>,
    language_: Option<Language>,
    read_: Option<bool>,
    repo: &'a mut dyn Repository<LibraryEntry>,
}

impl<'a> AddCommandBuilder<'a> {
    pub fn title(&mut self, title: String) -> &Self {
        self.title_ = Some(title);
        self
    }

    pub fn author(&mut self, author: String) -> &Self {
        self.author_ = Some(author);
        self
    }

    pub fn edition(&mut self, edition: String) -> &Self {
        self.edition_ = Some(edition);
        self
    }

    pub fn genre(&mut self, genre: Genre) -> &Self {
        self.genre_ = Some(genre);
        self
    }

    pub fn language(&mut self, language: Language) -> &Self {
        self.language_ = Some(language);
        self
    }

    pub fn read(&mut self, read: bool) -> &Self {
        self.read_ = Some(read);
        self
    }

    pub fn build(self) -> AddCommand<'a> {
        AddCommand {
            repo: self.repo,
            title: self.title_,
            author: self.author_,
            genre: self.genre_,
            edition: self.edition_,
            language: self.language_,
            read: self.read_,
        }
    }
}

impl Command for AddCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing add command with parameter");
        if let Some(title) = &self.title {
            println!("Adding Entry with title {}", title);
        }

        if let Some(author) = &self.author {
            println!("Adding Entry with author {}", author);
        }
        self.repo.create(LibraryEntry::default());
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undo to add command");
        true
    }
}
