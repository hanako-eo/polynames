use serde::Serialize;

use super::Model;

#[derive(Debug, Serialize)]
pub struct Word {
    pub id: u64,
    pub word: String,
}

impl Model for Word {
    const NUM_COLUMN: usize = 2;
    const TABLE: &'static str = "words";

    fn from_row(row: &duckdb::Row) -> duckdb::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            word: row.get(1)?,
        })
    }

    fn into_row(&self) -> impl AsRef<[&dyn duckdb::ToSql]> {
        [&self.id, &self.word] as [&dyn duckdb::ToSql; Self::NUM_COLUMN]
    }

    async fn update(&mut self) -> duckdb::Result<usize> {
        unimplemented!()
    }
}
