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
    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title1", "Author1", "Edition1", NaiveDate::default()),
            Genre::horror,
            Language::french,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title2", "Author1", "Edition1", NaiveDate::default()),
            Genre::horror,
            Language::french,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title3", "Author1", "Edition1", NaiveDate::default()),
            Genre::horror,
            Language::french,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title4", "Author2", "Edition2", NaiveDate::default()),
            Genre::crime,
            Language::french,
        ),
        BookMetadata {
            start_read_date: NaiveDate::default(),
            finish_read_date: NaiveDate::default(),
            read: true,
            borrowed: false,
        },
    );
    repo.create(library_entry);

    let library_entry = LibraryEntry::new(
        Book::new(
            CoverInfo::new("Title5", "Author3", "Edition1", NaiveDate::default()),
            Genre::horror,
            Language::french,
        ),
        BookMetadata::default(),
    );
    repo.create(library_entry);
}
