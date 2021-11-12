use crate::controller::WebState;
use actix_web::{HttpResponse, web::{self, Json}};
use domain::model::person::Person;


pub async fn insert(container: web::Data<WebState>, body: Json<Person>) -> HttpResponse {
    println!("JSON Payload {:?}", body);

    match container.command_person.insert(body.into_inner()){
        Ok(dni) => HttpResponse::Ok().body("Registro con exito:".to_string()+&dni),
        Err(_e) => HttpResponse::InternalServerError().body(_e.to_string()),
    }
}
