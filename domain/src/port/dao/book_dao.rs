use crate::model::book::{Book, CodeBook};
use anyhow::Error;

pub trait BookDao {
    fn find(&self, code_book: CodeBook) -> Result<Option<Book>, Error>;
}