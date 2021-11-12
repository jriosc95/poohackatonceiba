use domain::model::person::{Person, PersonNit};
use domain::port::repository::person_repository::PersonRepository;
use include_sql::include_sql;
use postgres::{Client, NoTls};
use crate::settings::Settings;

include_sql!("infraestructure/src/resources/person.sql", "$");

pub struct PersonRepositoryImpl {
    
}

impl PersonRepository for PersonRepositoryImpl {
    fn delete(&self, dni: PersonNit) -> Result<String, anyhow::Error> {
        todo!()
    }

    fn insert(&self, person: Person) -> Result<String, anyhow::Error> {
        match self.create_person(&person){
            Ok(dni) => Ok(dni),
            Err(e) => Err(e)
        }
    }

    fn update(&self, person: Person) -> Result<String, anyhow::Error> {
        todo!()
    }
}

impl PersonRepositoryImpl {
    pub fn new() -> PersonRepositoryImpl {
        PersonRepositoryImpl {}
    }

    fn create_person(&self, person:&Person) -> Result<String, anyhow::Error> {
        let settings = Settings::load().expect("Failed to load configuration.");
        let mut conn = Client::connect(&settings.db.connect_string.to_string(), NoTls)?;

        conn.execute(INSERT_PERSON,  &[&person.nit, &person.name, &person.date_of_birth],)?;

        Ok(person.nit.to_string())
    }
}
