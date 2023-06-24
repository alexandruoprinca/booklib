use super::Repository;
use crate::library_entry::LibraryEntry;

pub struct LibraryEntriesRepository {
    data: Vec<LibraryEntry>,
}

impl LibraryEntriesRepository {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Repository<LibraryEntry> for LibraryEntriesRepository {
    fn find(&self, id: u64) -> Option<LibraryEntry> {
        if let Some(position) = self.data.iter().position(|x| x.id() == id) {
            return Some(self.data[position].clone());
        }
        None
    }

    fn create(&mut self, item: LibraryEntry) {
        self.data.push(item);
    }

    fn update(&mut self, item: LibraryEntry) {}

    fn delete(&mut self, id: u64) {
        if let Some(position) = self.data.iter().position(|x| x.id() == id) {
            self.data.remove(position);
            
        }
    }

    fn get_all(&self) -> &Vec<LibraryEntry> {
        &self.data
    }
}
