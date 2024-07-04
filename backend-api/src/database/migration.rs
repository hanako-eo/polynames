use std::fs;
use std::path::{Path, PathBuf};

use duckdb::Error as DuckDBError;
use tokio::sync::RwLock;

use crate::models::internal::migration::Migration;
use crate::models::{Model, Ordering};
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

    let path = Path::new("./")
        .canonicalize()
        .map_err(|_| MigrationError::GetPathError)?
        .join("migrations");

    if !path.is_dir() {
        return Err(MigrationError::NotADirectory(path));
    }

    {
        let connection = connection.write().await;
        create_table_migrate(&connection)
            .await
            .map_err(|err| MigrationError::DuckDBError(origin.clone(), err))?;
    }

    for entry in fs::read_dir(&path).map_err(|_| MigrationError::NotADirectory(path))? {
        let entry = entry.map_err(|_| MigrationError::EntryFileError)?;
        let path = entry.path();

        let file_name = match (
            path.file_name().and_then(|os_str| os_str.to_str()),
            path.extension().and_then(|os_str| os_str.to_str()),
        ) {
            (Some(file_name), Some("sql")) => file_name,
            _ => continue,
        };

        let mut migration = Migration::where_one("file_name", Ordering::Equal, file_name)
            .await
            .map_err(|err| MigrationError::DuckDBError(origin.clone(), err))?;

        if !migration.as_ref().map(|m| m.migrated).unwrap_or_default() {
            {
                let request =
                    fs::read_to_string(&path).map_err(|_| MigrationError::EntryFileError)?;
                let connection = connection.write().await;
                connection
                    .execute_batch(request.as_str())
                    .map_err(|err| MigrationError::DuckDBError(path.clone(), err))?;
            }

            match migration {
                Some(ref mut migration) => migration
                    .update()
                    .await
                    .map_err(|err| MigrationError::DuckDBError(origin.clone(), err)),
                None => Migration::create(Migration {
                    file_name: file_name.to_string(),
                    migrated: true,
                })
                .await
                .map_err(|err| MigrationError::DuckDBError(origin.clone(), err)),
            }?;
        }
    }

    Ok(())
}
