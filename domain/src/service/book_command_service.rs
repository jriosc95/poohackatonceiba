use anyhow::Error;
use crate::model::book::CodeBook;
use crate::model::book::Book;
use crate::port::repository::book_repository::BookRepository;

pub struct BookCommandService {
    pub book_repository: Box<dyn BookRepository>,
}

impl BookRepository for BookCommandService {
    // Business logic goes here
    fn insert(&self, book: Book) -> Result<String, Error> {
        self.book_repository.insert(book)
    }
    fn update(&self, book: Book) -> Result<String, Error> {
        self.book_repository.update(book)
    }
    fn delete(&self, code: CodeBook) -> Result<String, Error> {
        self.book_repository.delete(code)
    }
}

impl BookCommandService {
    pub fn new(book_repository: Box<dyn BookRepository>) -> BookCommandService {
        BookCommandService { book_repository }
    }
}