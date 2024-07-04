#[macro_export]
macro_rules! match_duckdb_error {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        }
    };
}
