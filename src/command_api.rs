use rocket::State;

use crate::{
    arguments_provider::ArgumentsProviderRequest,
    command::{CommandType, ListCommand},
    command_factory::CommandFactory,
    list_output_handler::{ConsoleOutputHandler, JsonOutputHandler, ListOutputHandler},
    App,
};

use rocket::response::content;

#[get("/?<author>&<read>")]
pub fn list(
    author: Option<String>,
    read: Option<bool>,
    state: &State<App>,
) -> content::RawJson<String> {
    let args = ArgumentsProviderRequest::new();
    println!("Author is {}", author.unwrap());
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
    let args = ArgumentsProviderRequest::new();
    let mut repo_lock = state.repo.lock().unwrap();
    let mut command = CommandFactory::create_add_command(repo_lock.as_mut(), Box::new(args));

    command.execute();
}
