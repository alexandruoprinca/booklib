use crate::{library_entry::LibraryEntry, repository::Repository};

use super::Command;

pub struct AddCommand<'a> {
    bookname: String,
    repo: &'a dyn Repository<LibraryEntry>,
}

impl AddCommand<'_> {
    pub fn new<'a>(bookname: String, repo: &'a dyn Repository<LibraryEntry>) -> AddCommand<'a> {
        AddCommand {
            bookname: bookname,
            repo: repo,
        }
    }

    pub fn bookname(&self) -> &str {
        &self.bookname
    }
}

impl Command for AddCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing add command with parameter {}", self.bookname);
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undo to add command");
        true
    }
}
