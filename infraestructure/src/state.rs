use crate::adapter::repository::person_repository_impl::PersonRepositoryImpl;
use crate::controller::WebState;
use domain::service::person_command_service::PersonCommandService;
use std::sync::Arc;

pub async fn state_factory() -> std::io::Result<WebState> {
    let person_repository = PersonRepositoryImpl::new();
    let command_person = PersonCommandService::new(Box::new(person_repository));
    let state = WebState {
        command_person: Arc::new(command_person), 
    };
    Ok(state)
}