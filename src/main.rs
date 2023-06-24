mod command;
mod command_factory;
mod library_entry;
mod repository;

use chrono::NaiveDate;
use clap::{arg, command, Command as ArgCommand};
use command::{AddCommand, Command};
use library_entry::{Book, BookMetadata, CoverInfo, Genre, Language, LibraryEntry};
use repository::LibraryEntriesRepository;

use crate::{command::CommandType, command_factory::CommandFactory, repository::Repository};

fn fillRepoWithDummyData(repo: &mut Box<dyn Repository<library_entry::LibraryEntry>>) {
    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title1", "Author1", "Edition1", NaiveDate::default()),
            Genre::Horror,
            Language::French,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title2", "Author1", "Edition1", NaiveDate::default()),
            Genre::Horror,
            Language::French,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title3", "Author1", "Edition1", NaiveDate::default()),
            Genre::Horror,
            Language::French,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title4", "Author2", "Edition2", NaiveDate::default()),
            Genre::Horror,
            Language::French,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title5", "Author3", "Edition1", NaiveDate::default()),
            Genre::Horror,
            Language::French,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);
}

fn main() {
    let mut repo: Box<dyn Repository<library_entry::LibraryEntry>> =
        Box::new(LibraryEntriesRepository::new());

    fillRepoWithDummyData(&mut repo);

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            ArgCommand::new("add")
                .about("Adds files to myapp")
                .arg(arg!(--"author" <String>))
                .arg(arg!(--"title" <String>))
                .arg(arg!(--"read")),
        )
        .subcommand(ArgCommand::new("list")
                .about("List books")
                .arg(arg!(--"author" <String>))
        )
        .get_matches();

    let mut command = match matches.subcommand() {
        Some(("add", matches)) => {
            CommandFactory::new_command(CommandType::ADD, repo.as_mut(), matches)
        }
        Some(("list", matches)) => {
            CommandFactory::new_command(CommandType::LIST, repo.as_mut(), matches)
        }
        _ => unreachable!("Wrong command"),
    };

    command.execute();

    // println!("{}", matches.get_one::<String>("author").unwrap());
    // println!("{}", matches.get_one::<String>("title").unwrap());
    // println!("{}", matches.get_one::<bool>("read").unwrap());
}
