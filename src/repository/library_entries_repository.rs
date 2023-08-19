use chrono::NaiveDate;

use super::Repository;
use crate::library_entry::*;

pub struct LibraryEntriesRepository {
    data: Vec<LibraryEntry>,
}

impl LibraryEntriesRepository {
    pub fn new() -> Self {
        let mut repo = Self { data: Vec::new() };
        fill_repo_with_dummy_data(&mut repo);
        repo
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

fn fill_repo_with_dummy_data(repo: &mut LibraryEntriesRepository) {
    let library_entry = LibraryEntry::new("Title1".to_string(), "Author1".to_string())
    .genre(Genre::horror)
    .language(Language::french)
    .read(false)
    .build();
    repo.create(library_entry);

    let library_entry = LibraryEntry::new("Title2".to_string(), "Author1".to_string())
    .genre(Genre::horror)
    .language(Language::french)
    .read(true)
    .build();
    repo.create(library_entry);

    let library_entry = LibraryEntry::new("Title3".to_string(), "Author1".to_string())
    .genre(Genre::horror)
    .language(Language::french)
    .build();
    repo.create(library_entry);

    let library_entry = LibraryEntry::new("Title4".to_string(), "Author2".to_string())
    .genre(Genre::crime)
    .language(Language::french)
    .read(true)
    .build();
    repo.create(library_entry);

    let library_entry = LibraryEntry::new("Title5".to_string(), "Author3".to_string())
    .genre(Genre::horror)
    .language(Language::french)
    .build();
    repo.create(library_entry);
}
