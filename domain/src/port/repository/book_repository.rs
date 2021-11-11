use crate::model::book::Book;
use crate::model::book::CodeBook;
use anyhow::Error;

pub trait BookRepository {

    fn insert(&self, book: Book) -> Result<String, Error>;

    fn update(&self, dni: Book) -> Result<String, Error>;

    fn delete(&self, code: CodeBook) -> Result<String, Error>;
}
