use strum_macros::IntoStaticStr;

use crate::{library_entry::LibraryEntry, repository::Repository};

use super::Command;

#[derive(IntoStaticStr)]
pub enum ListCommandArgs {
    author,
}

pub struct ListCommand<'a> {
    repo: &'a dyn Repository<LibraryEntry>,
    author: Option<String>,
}

impl ListCommand<'_> {
    pub fn new<'a>(repo: &'a dyn Repository<LibraryEntry>) -> ListCommandBuilder<'a> {
        ListCommandBuilder {
            repo,
            author_: None,
        }
    }
}

pub struct ListCommandBuilder<'a> {
    author_: Option<String>,
    repo: &'a dyn Repository<LibraryEntry>,
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
        }
    }
}

impl Command for ListCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing list command");
        println!("Repo size is {}", self.repo.get_all().len());

        if let Some(author) = &self.author {
            for entry in self.repo.get_all() {
                if entry.book.cover_info.author == *author {
                    println!("{:?}", entry);
                }
            }
        }
        true
    }

    fn undo(&mut self) -> bool {
        println!("Undoing list command");
        true
    }
}
