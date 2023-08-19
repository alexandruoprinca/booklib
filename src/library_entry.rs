use chrono::NaiveDate;
use rocket::{http::RawStr, request::FromParam};
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

//TODO: Alex
// this can be remade as a procedural macro FromString that gets an enum from a string

#[derive(Default, Debug, Clone, Serialize, Display, EnumIter, PartialEq, Copy)]
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

#[derive(Default, Debug, Clone, Serialize, Display, EnumIter, PartialEq, Copy)]
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
    pub edition: Option<String>,
    pub release_date: Option<NaiveDate>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct Book {
    pub cover_info: CoverInfo,
    pub genre: Option<Genre>,
    pub language: Option<Language>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct BookMetadata {
    pub start_read_date: Option<NaiveDate>,
    pub finish_read_date: Option<NaiveDate>,
    pub read: Option<bool>,
    pub borrowed: Option<bool>,
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
            edition: Some(String::from(edition)),
            release_date: Some(release_date),
        }
    }
}

impl Book {
    pub fn new(cover_info: CoverInfo, genre: Genre, language: Language) -> Self {
        Self {
            cover_info,
            genre: Some(genre),
            language: Some(language),
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
            start_read_date: Some(start_read_date),
            finish_read_date : Some(finish_read_date),
            read : Some(read),
            borrowed: Some(borrowed)
        }
    }
}

//TODO: implement a builder for the Library entry
pub struct LibraryEntryBuilder{
    title: String,
    author: String,
    genre: Option<Genre>,
    language: Option<Language>,
    read: Option<bool>,
}

impl LibraryEntryBuilder {
    pub fn genre(&mut self, genre: Genre) -> &mut Self{
        self.genre = Some(genre);
        self
    }

    pub fn language(&mut self, language: Language) -> &mut Self{
        self.language = Some(language);
        self
    }

    pub fn read(&mut self, read: bool) -> &mut Self{
        self.read = Some(read);
        self
    }

    pub fn build(&mut self) -> LibraryEntry{
        let mut book = Book::default();
        book.genre = self.genre;
        book.language = self.language;
        book.cover_info.author = self.author.clone();
        book.cover_info.title = self.title.clone();

        let mut metadata = BookMetadata::default();
        metadata.read = self.read;

        LibraryEntry{
            id: LibraryEntry::id_generator(),
            book,
            metadata: metadata,
        }
    }

    fn new(title: String, author: String) -> Self {
        Self {
            title,
            author,
            genre: None,
            language: None,
            read: None,
        }
    }
}

impl LibraryEntry {
    pub fn new(title: String, author: String) -> LibraryEntryBuilder {
        LibraryEntryBuilder::new(title, author)
    }

    // pub fn new(book: Book, metadata: BookMetadata) -> Self {
    //     Self {
    //         id: Self::id_generator(),
    //         book,
    //         metadata,
    //     }
    // }

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
