use serde::Deserialize;
use serde::Serialize;

pub type PersonNit = String;
pub type PersonName = String;
pub type DateBirthPerson = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub nit: PersonNit,
    pub name: PersonName,
    pub date_of_birth: String,
}
