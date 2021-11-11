use config::{Config, ConfigError, File};
use include_sql::include_sql;
use postgres::{Client, Error, NoTls};
use serde::Deserialize;


include_sql!("domain/src/resources/schema.sql", "$");

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub web: Web,
    pub db: DB,
}

#[derive(Debug, Deserialize)]
pub struct Web {
    pub port: u16,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DB {
    pub connect_string: String,
}

#[allow(unused)]
impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name("./configuration/src/application.yaml"))?;

        println!("debug: {:?}", settings.get_bool("debug"));
        println!("web: {:?}", settings.get::<Web>("web"));
        println!("url conection:{:?}", settings.get::<DB>("db"));

        let schema = create_schema(&settings.get::<DB>("db").unwrap().connect_string);
        println!("shcema:{:?}", schema);

        settings.try_into()
    }
}

#[allow(unused)]
pub fn create_schema(connection: &str) -> Result<(), Error> {
    let mut conn = Client::connect(connection, NoTls)?;
    println!("SCHEMA: {}", SCHEMA);
    conn.batch_execute(SCHEMA)
}
