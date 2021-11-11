use crate::controller::health_controller::status;
use crate::controller::person_controller::insert;
use crate::state::state_factory;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .data_factory(state_factory)
            .route("/status", web::get().to(status))
            .route("/person/create", web::post().to(insert))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
