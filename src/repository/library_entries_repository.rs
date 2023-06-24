use super::Repository;
use crate::library_entry::LibraryEntry;

pub struct LibraryEntriesRepository {}

impl LibraryEntriesRepository {
    pub fn new() -> Self {
        Self {}
    }
}

impl Repository<LibraryEntry> for LibraryEntriesRepository {
    fn find(&self, id: u64) -> Option<LibraryEntry> {
        Some(LibraryEntry::default())
    }

    fn create(&mut self, item: LibraryEntry) {}

    fn update(&mut self, item: LibraryEntry) {}

    fn delete(&mut self, id: u64) {}

    fn get_all(&self) -> Vec<LibraryEntry> {
        Vec::new()
    }
}
