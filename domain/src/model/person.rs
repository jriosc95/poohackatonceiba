use datetime::LocalDate;

pub type PersonNit = String;
pub type PersonName = String;
pub type DateBirthPerson = LocalDate;

pub struct Person {
    pub nit: PersonNit,
    pub name: PersonName,
    pub date_of_birth: DateBirthPerson,
}
