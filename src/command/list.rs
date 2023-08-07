use std::error::Error;

use strum_macros::IntoStaticStr;

use super::Command;
use crate::list_output_handler::{ConsoleOutputHandler, ListOutputHandler};
use crate::{library_entry::LibraryEntry, repository::Repository};

#[derive(IntoStaticStr)]
pub enum ListCommandArgs {
    author,
}

pub struct ListCommand<'a> {
    repo: &'a dyn Repository<LibraryEntry>,
    author: Option<String>,
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
            output_handler_: output_handler,
        }
    }
}

pub struct ListCommandBuilder<'a> {
    author_: Option<String>,
    repo: &'a dyn Repository<LibraryEntry>,
    output_handler_: &'a mut dyn ListOutputHandler,
}

impl<'a> ListCommandBuilder<'a> {
    pub fn by_author(mut self, author: String) -> Self {
        self.author_ = Some(author);
        self
    }

    pub fn build(self) -> ListCommand<'a> {
        ListCommand {
            repo: self.repo,
            author: self.author_,
            output_handler: self.output_handler_,
        }
    }
}

impl Command for ListCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing list command");
        println!("Repo size is {}", self.repo.get_all().len());
        let mut result: Vec<LibraryEntry> = Vec::new();
        // if let Some(author) = &self.author {
        //     for entry in self.repo.get_all() {
        //         if entry.book.cover_info.author == *author {
        //             println!("{:?}", entry);
        //         }
        //     }
        // }
        for entry in self.repo.get_all() {
            result.push(entry.clone());
        }
        self.output_handler.handle_list_output(&result);
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undoing list command");
        true
    }
}
