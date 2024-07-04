use duckdb::params;
use serde::Serialize;

use crate::database::connection::get_connection;
use crate::models::Model;

#[derive(Serialize)]
pub struct Migration {
    pub file_name: String,
    pub migrated: bool,
}

impl Model for Migration {
    const NUM_COLUMN: usize = 2;
    const TABLE: &'static str = "__migrations";

    fn from_row(row: &duckdb::Row) -> duckdb::Result<Self> {
        Ok(Self {
            file_name: row.get(0)?,
            migrated: row.get(1)?,
        })
    }

    fn into_row(&self) -> impl AsRef<[&dyn duckdb::ToSql]> {
        [&self.file_name, &self.migrated] as [&dyn duckdb::ToSql; Self::NUM_COLUMN]
    }

    async fn update(&mut self) -> duckdb::Result<usize> {
        let connection = get_connection().write().await;

        let mut query = connection.prepare(
            format!(
                "UPDATE {} SET migrated = ? WHERE file_name = ?",
                Self::TABLE
            )
            .as_str(),
        )?;
        query.execute(params![self.migrated, self.file_name])
    }
}
