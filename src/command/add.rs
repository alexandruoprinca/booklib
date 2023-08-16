extern crate strum;
use strum_macros::IntoStaticStr;

use crate::{
    library_entry::{Genre, Language, LibraryEntry},
    repository::Repository,
};

use super::Command;
pub enum AddCommandOptions {
    title(String),
    author(String),
    genre(Genre),
    edition(String),
    language(Language),
    read(bool),
}

pub struct AddCommand<'a> {
    repo: &'a mut dyn Repository<LibraryEntry>,
    options: Vec<AddCommandOptions>,
}

impl AddCommand<'_> {
    pub fn new<'a>(repo: &'a mut dyn Repository<LibraryEntry>) -> AddCommandBuilder<'a> {
        AddCommandBuilder {
            repo,
            command_options: Vec::default(),
        }
    }
}

pub struct AddCommandBuilder<'a> {
    command_options: Vec<AddCommandOptions>,
    repo: &'a mut dyn Repository<LibraryEntry>,
}

impl<'a> AddCommandBuilder<'a> {
    pub fn build(self) -> AddCommand<'a> {
        AddCommand {
            repo: self.repo,
            options: self.command_options,
        }
    }

    pub fn add_option(&mut self, option: AddCommandOptions) {
        self.command_options.push(option);
    }
}

impl Command for AddCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing add command with parameter");
        self.repo.create(LibraryEntry::default());
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undo to add command");
        true
    }
}
