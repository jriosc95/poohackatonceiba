use crate::controller::WebState;
use actix_web::{HttpResponse, web};
use domain::model::person::Person;


pub async fn insert(container: web::Data<WebState>, body: String) -> HttpResponse {
    println!("JSON Payload {:?}", body);
 
    let deserialized: Person = serde_json::from_str(&body).unwrap();
    println!("JSON Payload {:?}", deserialized);

    match container.command_person.insert(deserialized){
        Ok(dni) => HttpResponse::Ok().body("Registro con exito:".to_string()+&dni),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
