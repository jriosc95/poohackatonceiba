use crate::model::person::Person;
use crate::model::person::PersonNit;
use anyhow::Error;

pub trait PersonRepository {

    fn insert(&self, person: Person) -> Result<String, Error>;

    fn update(&self, person: Person) -> Result<String, Error>;

    fn delete(&self, dni: PersonNit) -> Result<String, Error>;
}
