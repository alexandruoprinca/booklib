use crate::{library_entry::LibraryEntry, repository::Repository};

use super::Command;

pub struct ListCommand<'a> {
    repo: &'a dyn Repository<LibraryEntry>,
    args: &'a clap::ArgMatches
}

impl ListCommand<'_> {
    pub fn new<'a>(repo: &'a dyn Repository<LibraryEntry>, args: &'a clap::ArgMatches) -> ListCommand<'a> {
        ListCommand { repo, args }
    }
}

impl Command for ListCommand<'_> {
    fn execute(&mut self) -> bool {
        println!("Executing list command");
        println!("Repo size is {}", self.repo.get_all().len());

        if let Some(author) =  self.args.get_one::<String>("author")
        {
            for entry in self.repo.get_all() {
                if entry.book.cover_info.author == *author{
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
