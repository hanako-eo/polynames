use std::os::unix::net::SocketAddr;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use confy::ConfyError;

mod config;

const CONFIG_PATH: &str = "config.toml";

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match confy::load_path::<config::Config>(CONFIG_PATH) {
        Ok(config) => config,
        Err(ConfyError::BadTomlData(err)) => panic!(
            "an error occured on the parsing of the file {CONFIG_PATH}:\n{}",
            err.message()
        ),
        Err(ConfyError::GeneralLoadError(err)) => panic!(
            "an error occured on the loading of the file {CONFIG_PATH}:\n{}",
            err.kind()
        ),
        Err(_) => {
            panic!("wrong data in the file, failed to load config, please check {CONFIG_PATH}")
        }
    };

    let address = format!("{}:{}", config.address, config.port);
    HttpServer::new(|| App::new().service(hello))
        .bind(address)?
        .run()
        .await
}
