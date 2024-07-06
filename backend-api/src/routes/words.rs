use actix_web::{get, web, HttpResponse, Responder};

use crate::match_duckdb_error;
use crate::models::word::Word;
use crate::models::Model;

#[get("/words")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(
        match_duckdb_error!(Word::all().await)
            .into_iter()
            .map(|w| w.word)
            .collect::<Vec<String>>(),
    )
}

#[get("/words/{number}")]
async fn get_random_list(size: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok().json(
        match_duckdb_error!(Word::take_random_words(size.into_inner()).await)
            .into_iter()
            .map(|w| w.word)
            .collect::<Vec<String>>(),
    )
}
