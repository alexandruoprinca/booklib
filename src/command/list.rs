use crate::{library_entry::LibraryEntry, repository::Repository};

use super::Command;

pub struct ListCommand<'a> {
    repo: &'a dyn Repository<LibraryEntry>,
}

impl ListCommand<'_> {
    pub fn new<'a>(repo: &'a dyn Repository<LibraryEntry>) -> ListCommand<'a> {
        ListCommand { repo: repo }
    }
}

impl Command for ListCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing list command");
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undoing list command");
        true
    }
}
