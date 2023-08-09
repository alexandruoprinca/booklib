mod arguments_provider;
mod command;
mod command_api;
mod command_factory;
mod library_entry;
mod list_output_handler;
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
