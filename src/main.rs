mod arguments_provider;
mod command;
mod command_factory;
mod library_entry;
mod repository;

use crate::{command::CommandType, command_factory::CommandFactory, repository::Repository};
use arguments_provider::ArgumentsProviderCLI;
use chrono::NaiveDate;
use clap::{arg, command, Command as ArgCommand};
use command::{AddCommand, Command};
use library_entry::{Book, BookMetadata, CoverInfo, Genre, Language, LibraryEntry};
use repository::LibraryEntriesRepository;
// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> String {

//     let mut repo: Box<dyn Repository<library_entry::LibraryEntry>> =
//         Box::new(LibraryEntriesRepository::new());
//     CommandFactory::create_list_command(repo, args);

//     "Test".to_string()
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }

fn main() {
    let mut repo: Box<dyn Repository<library_entry::LibraryEntry>> =
        Box::new(LibraryEntriesRepository::new());

    fill_repo_with_dummy_data(&mut repo);

    let matches: clap::ArgMatches = command!()
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
        .subcommand(
            ArgCommand::new("list")
                .about("List books")
                .arg(arg!(--"author" <String>)),
        )
        .get_matches();

    let mut command = match matches.subcommand() {
        Some(("add", matches)) => {
            let command_args = arguments_provider::ArgumentsProviderCLI::new(matches.clone());
            CommandFactory::new_command(CommandType::ADD, repo.as_mut(), Box::new(command_args))
        }
        Some(("list", matches)) => {
            let command_args = arguments_provider::ArgumentsProviderCLI::new(matches.clone());
            CommandFactory::new_command(CommandType::LIST, repo.as_mut(), Box::new(command_args))
        }
        _ => unreachable!("Wrong command"),
    };

    command.execute();

    // println!("{}", matches.get_one::<String>("author").unwrap());
    // println!("{}", matches.get_one::<String>("title").unwrap());
    // println!("{}", matches.get_one::<bool>("read").unwrap());
}

fn fill_repo_with_dummy_data(repo: &mut Box<dyn Repository<library_entry::LibraryEntry>>) {
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
