mod arguments_provider;
mod command;
mod command_api;
mod command_factory;
mod library_entry;
mod repository;

use std::sync::Mutex;

use crate::repository::Repository;
use repository::LibraryEntriesRepository;
#[macro_use]
extern crate rocket;

pub struct App {
    repo: std::sync::Mutex<Box<dyn Repository<library_entry::LibraryEntry>>>,
}

#[launch]
fn rocket() -> _ {
    let repo: Box<dyn Repository<library_entry::LibraryEntry>> =
        Box::new(LibraryEntriesRepository::new());

    let state = App {
        repo: Mutex::new(repo),
    };

    let _rocket = rocket::build()
        .mount("/list", routes![command_api::list])
        .mount("/add", routes![command_api::add])
        .manage(state);
    _rocket
}

// let matches: clap::ArgMatches = command!()
//     .propagate_version(true)
//     .subcommand_required(true)
//     .arg_required_else_help(true)
//     .subcommand(
//         ArgCommand::new("add")
//             .about("Adds files to myapp")
//             .arg(arg!(--"author" <String>))
//             .arg(arg!(--"title" <String>))
//             .arg(arg!(--"read")),
//     )
//     .subcommand(
//         ArgCommand::new("list")
//             .about("List books")
//             .arg(arg!(--"author" <String>)),
//     )
//     .get_matches();

// let mut command = match matches.subcommand() {
//     Some(("add", matches)) => {
//         let command_args = arguments_provider::ArgumentsProviderCLI::new(matches.clone());
//         CommandFactory::new_command(CommandType::ADD, repo.as_mut(), Box::new(command_args))
//     }
//     Some(("list", matches)) => {
//         let command_args = arguments_provider::ArgumentsProviderCLI::new(matches.clone());
//         CommandFactory::new_command(CommandType::LIST, repo.as_mut(), Box::new(command_args))
//     }
//     _ => unreachable!("Wrong command"),
// };

// command.execute();
