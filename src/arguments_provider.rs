mod cli;
mod request;

pub use cli::ArgumentsProviderCLI;
pub use request::ArgumentsProviderRequest;

pub trait ArgumentsProvider {
    fn get_argument_string(&self, id: &str) -> Option<&String>;
    fn argument_exists(&self, id: &str) -> bool;
}
