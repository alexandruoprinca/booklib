use super::ArgumentsProvider;

pub struct ArgumentsProviderCLI {
    args: clap::ArgMatches,
}

impl ArgumentsProviderCLI {
    pub fn new(args: clap::ArgMatches) -> Self {
        Self { args }
    }
}

impl ArgumentsProvider for ArgumentsProviderCLI {
    fn get_argument_string(&self, id: &str) -> Option<&String> {
        self.args.get_one::<String>(id)
    }

    fn argument_exists(&self, id: &str) -> bool {
        self.args.contains_id(id)
    }
}
