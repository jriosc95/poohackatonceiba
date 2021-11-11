use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{HttpServer, App, web};
use crate::controller::health_controller::status;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/status", web::get().to(status))
    })
        .listen(listener)?
        .run();
    Ok(server)
}