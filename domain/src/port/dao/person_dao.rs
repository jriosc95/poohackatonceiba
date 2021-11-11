use crate::model::person::{Person, PersonNit};
use anyhow::Error;

pub trait PersonDao {
    fn find(&self, dni: PersonNit) -> Result<Option<Person>, Error>;
}