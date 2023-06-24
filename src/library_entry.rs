use chrono::NaiveDate;

#[derive(Default, Debug, Clone)]
pub enum Genre {
    Horror,
    Fantasy,
    #[default]
    Crime,
}

#[derive(Default, Debug, Clone)]
pub enum Language {
    English,
    Romanian,
    #[default]
    French,
}

#[derive(Default, Debug, Clone)]
pub struct CoverInfo {
    pub title: String,
    pub author: String,
    edition: String,
    release_date: NaiveDate,
}

#[derive(Default, Debug, Clone)]
pub struct Book {
    pub cover_info: CoverInfo,
    genre: Genre,
    language: Language,
}

#[derive(Default, Debug, Clone)]
pub struct BookMetadata {
    start_read_date: NaiveDate,
    finish_read_date: NaiveDate,
    read: bool,
    borrowed: bool,
}

#[derive(Default, Debug, Clone)]
pub struct LibraryEntry {
    id: u64,
    pub book: Book,
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

fn id_generator() -> u64 {
    static mut CURRENT_VALUE: u64 = 0;
    unsafe {
        CURRENT_VALUE += 1;
        CURRENT_VALUE
    }
}

impl LibraryEntry {
    pub fn new(book: Book, metadata: BookMetadata) -> Self {
        Self {
            id: id_generator(),
            book,
            metadata,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}
