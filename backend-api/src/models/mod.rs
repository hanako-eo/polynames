use duckdb::{Result, Row, ToSql};

use crate::database::connection::get_connection;
use crate::utils::Flatten;

pub mod internal;
pub mod word;

#[derive(Clone, Copy)]
pub enum Ordering {
    Equal,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

pub trait Model
where
    Self: Sized,
{
    const TABLE: &'static str;
    const NUM_COLUMN: usize;

    fn from_row(row: &Row) -> Result<Self>;
    fn into_row(&self) -> impl AsRef<[&dyn ToSql]>;

    async fn all() -> Result<Vec<Self>> {
        let connection = get_connection().read().await;

        let mut query = connection.prepare(&format!("SELECT * FROM {}", Self::TABLE))?;
        query
            .query_map([], Self::from_row)
            .map(|rows| rows.collect::<Result<Vec<Self>>>())
            .flatten()
    }

    async fn where_all<T: ToSql>(column: &str, op: Ordering, value: T) -> Result<Vec<Self>> {
        let connection = get_connection().read().await;

        let mut query = connection.prepare(
            format!(
                "SELECT * FROM {} WHERE {column} {} ?",
                Self::TABLE,
                op.as_str()
            )
            .as_str(),
        )?;
        query
            .query_map([value], Self::from_row)
            .map(|rows| rows.collect::<Result<Vec<Self>>>())
            .flatten()
    }

    async fn where_many<T: ToSql>(column: &str, op: Ordering, values: Vec<T>) -> Result<Vec<Self>> {
        let connection = get_connection().read().await;

        let mut query = connection.prepare(
            format!("SELECT * FROM {} WHERE {column} {} ? LIMIT 1", Self::TABLE, op.as_str()).as_str(),
        )?;

        let mut result = Vec::with_capacity(values.len());
        for value in values {
            result.extend(
                query
                    .query_map([value], Self::from_row)
                    .map(|rows| rows.collect::<Result<Vec<Self>>>())
                    .flatten()?,
            );
        }

        Ok(result)
    }

    async fn where_one<T: ToSql>(column: &str, op: Ordering, value: T) -> Result<Option<Self>> {
        let connection = get_connection().read().await;

        let mut query = connection.prepare(
            format!(
                "SELECT * FROM {} WHERE {column} {} ? LIMIT 1",
                Self::TABLE,
                op.as_str()
            )
            .as_str(),
        )?;
        query
            .query_map([value], Self::from_row)
            .map(|rows| rows.collect::<Result<Vec<Self>>>())
            .flatten()
            .map(|mut list| list.pop())
    }

    async fn create(self: Self) -> Result<usize> {
        let connection = get_connection().write().await;

        let mut query = connection.prepare(&format!(
            "INSERT INTO {} VALUES ({})",
            Self::TABLE,
            vec!["?"; Self::NUM_COLUMN].join(",")
        ))?;
        query.execute(self.into_row().as_ref())
    }

    async fn update(&mut self) -> Result<usize>;
}

impl Ordering {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "=",
            Self::Greater => ">",
            Self::Less => "<",
            Self::GreaterEqual => ">=",
            Self::LessEqual => "<=",
        }
    }
}
