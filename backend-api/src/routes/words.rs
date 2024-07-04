use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Range;

use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;

use crate::database::connection::get_connection;
use crate::match_duckdb_error;
use crate::models::word::Word;
use crate::models::{Model, Ordering};

#[get("/words")]
async fn index() -> impl Responder {
    let words = match_duckdb_error!(Word::all().await);

    HttpResponse::Ok().json(words.into_iter().map(|w| w.word).collect::<Vec<String>>())
}

#[get("/words/{number}")]
async fn get_random_list(size: web::Path<usize>) -> impl Responder {
    let connection = get_connection().read().await;

    let length = match_duckdb_error!(connection.query_row(
        "SELECT currval('word_id_sequence') as words_len",
        [],
        // I do -1 because currval gives me the next index to give
        |row| row.get::<usize, u64>(0).map(|l| l - 1)
    ));

    let ids = sample(size.into_inner(), 1..(length + 1));

    HttpResponse::Ok().json(
        match_duckdb_error!(Word::where_many("id", Ordering::Equal, ids).await)
            .into_iter()
            .map(|w| w.word)
            .collect::<Vec<String>>(),
    )
}

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
