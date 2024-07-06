use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Range;

use duckdb::{Result, Row, ToSql};
use rand::Rng;
use serde::Serialize;

use super::{Model, Ordering};
use crate::database::connection::get_connection;

#[derive(Debug, Serialize)]
pub struct Word {
    pub id: u64,
    pub word: String,
}

impl Word {
    pub async fn take_random_words(size: usize) -> Result<Vec<Word>> {
        let connection = get_connection().read().await;

        // /!\ currval gives me the next index to give
        let length = connection.query_row(
            "SELECT currval('word_id_sequence') as words_len",
            [],
            |row| row.get::<usize, u64>(0),
        )?;

        Word::where_many("id", Ordering::Equal, Self::sample(size, 1..length)).await
    }

    // can be improve
    fn sample(size: usize, mut range: Range<u64>) -> Vec<u64> {
        assert!((size as u64) <= range.end - range.start);

        let mut rng = rand::thread_rng();

        let mut heap: BinaryHeap<Reverse<u64>> = BinaryHeap::with_capacity(size);
        let mut vector = Vec::with_capacity(size);

        for _ in 0..size {
            let mut number = rng.gen_range(range.clone());

            for exclude_number in heap.iter() {
                if number > (*exclude_number).0 {
                    break;
                }
                number += 1;
            }

            heap.push(Reverse(number));
            vector.push(number);

            range.end -= 1;
        }

        vector
    }
}

impl Model for Word {
    const NUM_COLUMN: usize = 2;
    const TABLE: &'static str = "words";

    fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            word: row.get(1)?,
        })
    }

    fn into_row(&self) -> impl AsRef<[&dyn ToSql]> {
        [&self.id, &self.word] as [&dyn ToSql; Self::NUM_COLUMN]
    }

    async fn update(&mut self) -> Result<usize> {
        unimplemented!()
    }
}
