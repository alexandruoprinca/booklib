use std::collections::HashMap;

use rocket::State;
use strum::IntoEnumIterator;

use crate::{
    command::{AddCommandOptions, CommandType, ListCommand, ListCommandOptions},
    command_factory::CommandFactory,
    library_entry::{Genre, Language},
    list_output_handler::{ConsoleOutputHandler, JsonOutputHandler, ListOutputHandler},
    App,
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
    let mut options: Vec<ListCommandOptions> = Vec::new();
    if let Some(x) = author {
        options.push(ListCommandOptions::author(x));
    }
    if let Some(x) = read {
        println!("received read {}", x);
        options.push(ListCommandOptions::read(x));
    }
    if let Some(x) = edition {
        options.push(ListCommandOptions::edition(x));
    }
    if let Some(x) = genre {
        for value in Genre::iter() {
            if value.to_string() == x {
                options.push(ListCommandOptions::genre(value));
                break;
            }
        }
    }
    if let Some(x) = language {
        for value in Language::iter() {
            if value.to_string() == x {
                options.push(ListCommandOptions::language(value));
                break;
            }
        }
    }

    let shared_data = state.inner();
    let mut repo_lock = shared_data.repo.lock().unwrap();
    let mut console_output_handler: Box<JsonOutputHandler> = Box::new(JsonOutputHandler::new());
    {
        let mut command = CommandFactory::create_list_command(
            repo_lock.as_mut(),
            options,
            console_output_handler.as_mut(),
        );

        command.execute();
    }
    let json = console_output_handler.get_json();

    rocket::response::content::RawJson(json.to_string())
}

#[get("/?<title>&<author>&<language>&<genre>&<read>&<edition>")]
pub fn add(
    title: Option<String>,
    author: Option<String>,
    language: Option<String>,
    genre: Option<String>,
    edition: Option<String>,
    read: Option<bool>,
    state: &State<App>,
) {
    let mut options: Vec<AddCommandOptions> = Vec::new();
    if let Some(x) = title {
        options.push(AddCommandOptions::title(x));
    }
    if let Some(x) = author {
        options.push(AddCommandOptions::author(x));
    }
    if let Some(x) = language {
        for value in Language::iter() {
            if value.to_string() == x {
                options.push(AddCommandOptions::language(value));
                break;
            }
        }
    }
    if let Some(x) = genre {
        for value in Genre::iter() {
            if value.to_string() == x {
                options.push(AddCommandOptions::genre(value));
                break;
            }
        }
    }
    if let Some(x) = read {
        println!("received read {}", x);
        options.push(AddCommandOptions::read(x));
    }
    if let Some(x) = edition {
        options.push(AddCommandOptions::edition(x));
    }

    let mut repo_lock = state.repo.lock().unwrap();
    let mut command = CommandFactory::create_add_command(repo_lock.as_mut(), options);

    command.execute();
}
