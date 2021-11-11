use anyhow::Error;
use crate::model::person::PersonNit;
use crate::model::person::Person;
use crate::port::repository::person_repository::PersonRepository;

pub struct PersonCommandService {
    pub person_repository: Box<dyn PersonRepository>,
}

impl PersonRepository for PersonCommandService {
    // Business logic goes here
    fn insert(&self, person: Person) -> Result<String, Error> {
        self.person_repository.insert(person)
    }
    fn update(&self, person: Person) -> Result<String, Error> {
        self.person_repository.update(person)
    }
    fn delete(&self, nit: PersonNit) -> Result<String, Error> {
        self.person_repository.delete(nit)
    }
}

impl PersonCommandService {
    pub fn new(person_repository: Box<dyn PersonRepository>) -> PersonCommandService {
        PersonCommandService { person_repository }
    }
}