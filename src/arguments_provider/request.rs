use std::collections::HashMap;

use super::ArgumentsProvider;

pub struct ArgumentsProviderRequest<'a> {
    args: HashMap<&'a str, String>,
}

impl<'a> ArgumentsProviderRequest<'a> {
    pub fn new(map: HashMap<&'a str, String>) -> ArgumentsProviderRequest<'a> {
      ArgumentsProviderRequest {
        args: map
      }
    }
}

impl ArgumentsProvider for ArgumentsProviderRequest<'_> {
    fn get_argument_string(&self, id: &str) -> Option<&String> {
        if self.args.contains_key(id) {
            return self.args.get(id);
        }
        None
    }

    fn argument_exists(&self, id: &str) -> bool {
        self.args.contains_key(id)
    }
}
