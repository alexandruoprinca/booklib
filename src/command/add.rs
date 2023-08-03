extern crate strum;
use strum_macros::IntoStaticStr;

use crate::{library_entry::LibraryEntry, repository::Repository};

use super::Command;

#[derive(IntoStaticStr)]
pub enum AddCommandArgs {
    title,
    author,
}

pub struct AddCommand<'a> {
    repo: &'a mut dyn Repository<LibraryEntry>,
    title: Option<String>,
    author: Option<String>,
}

impl AddCommand<'_> {
    pub fn new<'a>(repo: &'a mut dyn Repository<LibraryEntry>) -> AddCommandBuilder<'a> {
        AddCommandBuilder {
            repo,
            title_: None,
            author_: None,
        }
    }
}

pub struct AddCommandBuilder<'a> {
    title_: Option<String>,
    author_: Option<String>,
    repo: &'a mut dyn Repository<LibraryEntry>,
}

impl<'a> AddCommandBuilder<'a> {
    pub fn title(mut self, title: String) -> Self {
        self.title_ = Some(title);
        self
    }

    pub fn author(mut self, author: String) -> Self {
        self.author_ = Some(author);
        self
    }

    pub fn build(self) -> AddCommand<'a> {
        AddCommand {
            repo: self.repo,
            title: self.title_,
            author: self.author_,
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
