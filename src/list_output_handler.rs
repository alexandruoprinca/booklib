use crate::library_entry::LibraryEntry;

pub trait ListOutputHandler {
    fn handle_list_output(&mut self, entries_to_be_printed: &Vec<LibraryEntry>);
}

pub struct ConsoleOutputHandler;

impl ListOutputHandler for ConsoleOutputHandler {
    fn handle_list_output(&mut self, entries_to_be_printed: &Vec<LibraryEntry>) {
        for entry in entries_to_be_printed {
            println!("{:?}", entry);
        }
    }
}

impl ConsoleOutputHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct JsonOutputHandler {
    json: serde_json::Value,
}

impl ListOutputHandler for JsonOutputHandler {
    fn handle_list_output(&mut self, entries_to_be_printed: &Vec<LibraryEntry>) {
        let json = serde_json::to_value(&entries_to_be_printed);
        self.json = json.ok().unwrap();
    }
}

impl JsonOutputHandler {
    pub fn get_json(&self) -> serde_json::Value {
        self.json.clone()
    }

    pub fn new() -> Self {
        Self {
            json: serde_json::Value::default(),
        }
    }
}
