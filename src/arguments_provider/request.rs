use super::ArgumentsProvider;

pub struct ArgumentsProviderRequest {
    data: String,
    data2: bool,
}

impl ArgumentsProviderRequest {
    pub fn new() -> Self {
        Self {
            data: "data".to_string(),
            data2: false,
        }
    }
}

impl ArgumentsProvider for ArgumentsProviderRequest {
    fn get_argument_string(&self, id: &str) -> Option<&String> {
        Some(&self.data)
    }

    fn get_argument_bool(&self, id: &str) -> Option<&bool> {
        Some(&self.data2)
    }

    fn argument_exists(&self, id: &str) -> bool {
        false
    }
}
