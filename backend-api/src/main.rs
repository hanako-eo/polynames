use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use confy::ConfyError;
use database::connection::{disconnect, establish};
use database::migration::{migrate, MigrationError};
use duckdb::Error as DuckDBError;
use env_logger::{init_from_env, Env};

mod config;
mod database;
mod macros;
mod models;
mod routes;
mod utils;

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

    let connection = match establish(config.db_path) {
        Ok(connection) => connection,
        Err(DuckDBError::InvalidPath(path)) => {
            panic!("invalid database file path '{}'", path.display())
        }
        Err(err) => panic!("duckdb internal error:\n{err}"),
    };

    match migrate(connection).await {
        Ok(()) => (),
        Err(MigrationError::DuckDBError(path, err)) => {
            panic!("duckdb error in the file '{}':\n{}", path.display(), err)
        }
        Err(MigrationError::NotADirectory(path)) => {
            panic!("'{}' is not a directory", path.display())
        }
        Err(MigrationError::EntryFileError) | Err(MigrationError::GetPathError) => panic!(
            "an error occured with a file in the migrations folder or with the current folder"
        ),
    }

    let address = format!("{}:{}", config.address, config.port);

    init_from_env(Env::default().default_filter_or("info"));

    println!("Launching the server on {address} !");
    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/api")
                .service(routes::words::index)
                .service(routes::words::get_random_list),
        )
    })
    .bind(address)?
    .run()
    .await?;

    disconnect();

    Ok(())
}
