use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

use duckdb::Error as DuckDBError;
use inline_colorization::*;
use tokio::sync::RwLock;

use crate::models::internal::migration::Migration;
use crate::models::{Model, Ordering as ModelOrdering};
use crate::utils::ConnectionWrapper;

#[derive(Debug)]
pub enum MigrationError {
    GetPathError,
    NotADirectory(PathBuf),
    EntryFileError,

    DuckDBError(PathBuf, DuckDBError),
}

async fn create_table_migrate(connection: &ConnectionWrapper) -> duckdb::Result<()> {
    connection.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS __migrations (
            file_name STRING,
            migrated BOOL
        )
    ",
    )
}

pub async fn migrate(connection: &RwLock<ConnectionWrapper>) -> Result<(), MigrationError> {
    let origin = Path::new(file!())
        .canonicalize()
        .map_err(|_| MigrationError::GetPathError)?;

    let migrations_path = Path::new("./")
        .canonicalize()
        .map_err(|_| MigrationError::GetPathError)?
        .join("migrations");

    if !migrations_path.is_dir() {
        return Err(MigrationError::NotADirectory(migrations_path));
    }

    {
        let connection = connection.write().await;
        create_table_migrate(&connection)
            .await
            .map_err(|err| MigrationError::DuckDBError(origin.clone(), err))?;
    }

    let mut entries = fs::read_dir(&migrations_path)
        .map_err(|_| MigrationError::NotADirectory(migrations_path))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            let file_name = match (
                path.file_name(),
                path.extension().and_then(|os_str| os_str.to_str()),
            ) {
                (Some(file_name), Some("sql")) => file_name.to_string_lossy(),
                _ => return None,
            };

            Some((path.clone(), file_name.to_string()))
        })
        .collect::<Vec<_>>();

    entries.sort_by(|(_, file_name_a), (_, file_name_b)| {
        let Some(date_a) = file_name_a
            .split_once('_')
            .and_then(|(data, _)| data.parse::<u64>().ok())
        else {
            return Ordering::Less;
        };
        let Some(date_b) = file_name_b
            .split_once('_')
            .and_then(|(data, _)| data.parse::<u64>().ok())
        else {
            return Ordering::Less;
        };

        date_a.cmp(&date_b)
    });

    for (path, file_name) in entries {
        let mut migration = Migration::where_one("file_name", ModelOrdering::Equal, &file_name)
            .await
            .map_err(|err| MigrationError::DuckDBError(origin.clone(), err))?;

        if !migration.as_ref().map(|m| m.migrated).unwrap_or_default() {
            {
                println!("{color_blue}migrating {file_name} ...{color_reset}");

                let request =
                    fs::read_to_string(&path).map_err(|_| MigrationError::EntryFileError)?;
                let connection = connection.write().await;
                connection
                    .execute_batch(request.as_str())
                    .map_err(|err| MigrationError::DuckDBError(path.to_path_buf(), err))?;

                println!(
                    "{color_green}migration of {file_name} completed successfully{color_reset}"
                );
            }

            match migration {
                Some(ref mut migration) => migration
                    .update()
                    .await
                    .map_err(|err| MigrationError::DuckDBError(origin.clone(), err)),
                None => Migration::create(Migration {
                    file_name,
                    migrated: true,
                })
                .await
                .map_err(|err| MigrationError::DuckDBError(origin.clone(), err)),
            }?;
        }
    }

    println!();

    Ok(())
}
