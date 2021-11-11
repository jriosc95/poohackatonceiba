#[warn(unused_variables)]
use std::net::TcpListener;
use infraestructure::settings::Settings;
use infraestructure::start::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::load().expect("Failed to load configuration.");
    let address = format!("{}:{}", settings.web.url, settings.web.port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await?;
    Ok(())
}

