pub mod book_controller;
use std::sync::Arc;
use domain::port::repository::person_repository::PersonRepository;
pub mod person_controller;
pub mod health_controller;

#[derive(Clone)]
pub struct WebState {
    pub command_person: Arc<dyn PersonRepository>,
}