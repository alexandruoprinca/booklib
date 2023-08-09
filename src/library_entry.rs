use chrono::NaiveDate;
use rocket::{http::RawStr, request::FromParam};
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

//TODO: Alex
// this can be remade as a procedural macro FromString that gets an enum from a string

#[derive(Default, Debug, Clone, Serialize, Display, EnumIter, PartialEq)]
pub enum Genre {
    horror,
    fantasy,
    #[default]
    crime,
}

impl Genre {
    pub fn from_string(id: &str) -> Option<Genre> {
        for value in Genre::iter() {
            if value.to_string() == id {
                return Some(value);
            }
        }
        None
    }
}

#[derive(Default, Debug, Clone, Serialize, Display, EnumIter, PartialEq)]
pub enum Language {
    english,
    romanian,
    #[default]
    french,
}

impl Language {
    pub fn from_string(id: &str) -> Option<Language> {
        for value in Language::iter() {
            if value.to_string() == id {
                return Some(value);
            }
        }
        None
    }
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct CoverInfo {
    pub title: String,
    pub author: String,
    pub edition: String,
    pub release_date: NaiveDate,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct Book {
    pub cover_info: CoverInfo,
    pub genre: Genre,
    pub language: Language,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct BookMetadata {
    pub start_read_date: NaiveDate,
    pub finish_read_date: NaiveDate,
    pub read: bool,
    pub borrowed: bool,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct LibraryEntry {
    id: u64,
    pub book: Book,
    pub metadata: BookMetadata,
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
        Self {
            id: Self::id_generator(),
            book,
            metadata,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    fn id_generator() -> u64 {
        static mut CURRENT_VALUE: u64 = 0;
        unsafe {
            CURRENT_VALUE += 1;
            CURRENT_VALUE
        }
    }
}
