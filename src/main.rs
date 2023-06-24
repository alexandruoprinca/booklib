mod command;
mod command_factory;
mod library_entry;
mod repository;

use clap::{arg, command, Command as ArgCommand};
use command::{AddCommand, Command};
use repository::LibraryEntriesRepository;

use crate::{command::CommandType, command_factory::CommandFactory, repository::Repository};
fn main() {
    let mut repo: Box<dyn Repository<library_entry::LibraryEntry>> =
        Box::new(LibraryEntriesRepository::new());

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
        .subcommand(ArgCommand::new("list").about("List books"))
        .get_matches();

    let mut command = match matches.subcommand() {
        Some(("add", matches)) => {
            CommandFactory::new_command(CommandType::ADD, repo.as_ref(), matches)
        }
        Some(("list", matches)) => {
            CommandFactory::new_command(CommandType::LIST, repo.as_ref(), matches)
        }
        _ => unreachable!("Wrong command"),
    };

    command.execute();

    // println!("{}", matches.get_one::<String>("author").unwrap());
    // println!("{}", matches.get_one::<String>("title").unwrap());
    // println!("{}", matches.get_one::<bool>("read").unwrap());
}
