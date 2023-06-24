use chrono::NaiveDate;

#[derive(Default)]
pub enum Genre {
    Horror,
    Fantasy,
    #[default]
    Crime,
}

#[derive(Default)]
pub enum Language {
    English,
    Romanian,
    #[default]
    French,
}

#[derive(Default, Debug)]
pub struct CoverInfo {
    title: String,
    author: String,
    edition: String,
    release_date: NaiveDate,
}

#[derive(Default)]
pub struct Book {
    cover_info: CoverInfo,
    genre: Genre,
    language: Language,
}

#[derive(Default)]
pub struct BookMetadata {
    start_read_date: NaiveDate,
    finish_read_date: NaiveDate,
    read: bool,
    borrowed: bool,
}

#[derive(Default)]
pub struct LibraryEntry {
    book: Book,
    metadata: BookMetadata,
}

impl CoverInfo {
    pub fn new(title: &str, author: &str, edition: &str, release_date: NaiveDate) -> Self {
        Self {
            title: String::from(title),
            author: String::from(author),
            edition: String::from(edition),
            release_date,
        }
    }
}

impl Book {
    pub fn new(cover_info: CoverInfo, genre: Genre, language: Language) -> Self {
        Self {
            cover_info,
            genre,
            language,
        }
    }
}

impl BookMetadata {
    pub fn new(
        start_read_date: NaiveDate,
        finish_read_date: NaiveDate,
        read: bool,
        borrowed: bool,
    ) -> Self {
        Self {
            start_read_date,
            finish_read_date,
            read,
            borrowed,
        }
    }
}

impl LibraryEntry {
    pub fn new(book: Book, metadata: BookMetadata) -> Self {
        Self { book, metadata }
    }
}
