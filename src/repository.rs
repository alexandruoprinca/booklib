mod library_entries_repository;

pub use library_entries_repository::LibraryEntriesRepository;

pub trait Repository<T> {
    fn find(&self, id: u64) -> Option<T>;
    fn create(&mut self, item: T);
    fn update(&mut self, item: T);
    fn delete(&mut self, id: u64);
    fn get_all(&self) -> &Vec<T>;
}
