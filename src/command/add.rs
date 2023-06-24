use crate::{library_entry::LibraryEntry, repository::Repository};

use super::Command;

pub struct AddCommand<'a> {
    repo: &'a mut dyn Repository<LibraryEntry>,
    args: &'a clap::ArgMatches
}

impl AddCommand<'_> {
    pub fn new<'a>(repo: &'a mut dyn Repository<LibraryEntry>, args: &'a clap::ArgMatches) -> AddCommand<'a> {
        AddCommand {
            repo,
            args
        }
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
