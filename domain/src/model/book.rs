use super::person::Person;

pub type CodeBook = String;
pub type NameBook = String;
pub type OwnerBook = Person;
pub type NumberSheets = u8;

pub struct Book {
    pub code: CodeBook,
    pub name: NameBook,
    pub owner: OwnerBook,
    pub number_of_sheets: NumberSheets,
}
