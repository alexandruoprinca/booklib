use std::collections::HashMap;

use rocket::State;
use strum::IntoEnumIterator;

use crate::{
    arguments_provider::ArgumentsProviderRequest,
    command::{AddCommandArgs, CommandType, ListCommand, ListCommandArgs},
    command_factory::CommandFactory,
    list_output_handler::{ConsoleOutputHandler, JsonOutputHandler, ListOutputHandler},
    App, library_entry::{Language, Genre},
};

use rocket::response::content;

#[get("/?<author>&<read>&<edition>&<genre>&<language>")]
pub fn list(
    author: Option<String>,
    read: Option<bool>,
    edition: Option<String>,
    genre: Option<String>,
    language: Option<String>,
    state: &State<App>,
) -> content::RawJson<String> {
    let mut map: HashMap<&str, String> = HashMap::default();
    if let Some(x) = author {
        map.insert(ListCommandArgs::author.into(), x);
    }
    if let Some(x) = read {
        println!("received read {}", x);
        map.insert(ListCommandArgs::read.into(), x.to_string());
    }
    if let Some(x) = edition {
        map.insert(ListCommandArgs::edition.into(), x);
    }
    if let Some(x) = genre {
        for value in Genre::iter() {
            if value.to_string() == x {
                map.insert(ListCommandArgs::genre.into(), x);
                break;
            }
        }
    }
    if let Some(x) = language {
        for value in Language::iter() {
            if value.to_string() == x {
                map.insert(ListCommandArgs::language.into(), x);
                break;
            }
        }
    }

    let args = ArgumentsProviderRequest::new(map);

    let shared_data = state.inner();
    let mut repo_lock = shared_data.repo.lock().unwrap();
    let mut console_output_handler: Box<JsonOutputHandler> = Box::new(JsonOutputHandler::new());
    {
        let mut command = CommandFactory::create_list_command(
            repo_lock.as_mut(),
            Box::new(args),
            console_output_handler.as_mut(),
        );

        command.execute();
    }
    let json = console_output_handler.get_json();

    rocket::response::content::RawJson(json.to_string())
}

#[get("/?<title>&<author>")]
pub fn add(title: Option<String>, author: Option<String>, state: &State<App>) {
    let mut map: HashMap<&str, String> = HashMap::default();
    if let Some(x) = title {
        map.insert(AddCommandArgs::title.into(), x);
    }
    if let Some(x) = author {
        map.insert(AddCommandArgs::author.into(), x);
    }
    let args = ArgumentsProviderRequest::new(map);

    let mut repo_lock = state.repo.lock().unwrap();
    let mut command = CommandFactory::create_add_command(repo_lock.as_mut(), Box::new(args));

    command.execute();
}
