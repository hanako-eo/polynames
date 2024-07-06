#![allow(unstable_name_collisions)]

use actix_web::{middleware, web, App, HttpServer};
use confy::ConfyError;
use database::connection::{disconnect, establish};
use database::migration::{migrate, MigrationError};
use duckdb::Error as DuckDBError;
use env_logger::{init_from_env, Env};
use inline_colorization::*;

mod config;
mod database;
mod macros;
mod models;
mod routes;
mod utils;

const CONFIG_PATH: &str = "config.toml";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = match confy::load_path::<config::Config>(CONFIG_PATH) {
        Ok(config) => config,
        Err(ConfyError::BadTomlData(err)) => panic!(
            "{color_red}an error occured on the parsing of the file {CONFIG_PATH}:\n{}{color_reset}",
            err.message()
        ),
        Err(ConfyError::GeneralLoadError(err)) => panic!(
            "{color_red}an error occured on the loading of the file {CONFIG_PATH}:\n{}{color_reset}",
            err.kind()
        ),
        Err(_) => {
            panic!("{color_red}wrong data in the file, failed to load config, please check {CONFIG_PATH}{color_reset}")
        }
    };

    let connection = match establish(config.db_path) {
        Ok(connection) => connection,
        Err(DuckDBError::InvalidPath(path)) => {
            panic!(
                "{color_red}invalid database file path '{}'{color_reset}",
                path.display()
            )
        }
        Err(err) => panic!("{color_red}duckdb internal error:\n{err}{color_reset}"),
    };

    match migrate(connection).await {
        Ok(()) => (),
        Err(MigrationError::DuckDBError(path, err)) => {
            panic!("{color_red}duckdb error in the file '{}':\n{}{color_reset}", path.display(), err)
        }
        Err(MigrationError::NotADirectory(path)) => {
            panic!("{color_red}'{}' is not a directory{color_reset}", path.display())
        }
        Err(MigrationError::EntryFileError) | Err(MigrationError::GetPathError) => panic!(
            "{color_red}an error occured with a file in the migrations folder or with the current folder{color_reset}"
        ),
    }

    let address = format!("{}:{}", config.address, config.port);

    init_from_env(Env::default().default_filter_or("info"));

    println!("{color_blue}Launching the server on {address} !{color_reset}");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(routes::words::index)
                    .service(routes::words::get_random_list),
            )
            .service(web::scope("/ws").service(routes::websocket::index))
    })
    .bind(address)?
    .run()
    .await?;

    disconnect();

    Ok(())
}
