use std::path::Path;
use std::sync::OnceLock;

use duckdb::Result;
use tokio::sync::RwLock;

use crate::utils::ConnectionWrapper;

static mut DB_CONNECTION: OnceLock<RwLock<ConnectionWrapper>> = OnceLock::new();

pub fn establish<P: AsRef<Path>>(path: P) -> Result<&'static RwLock<ConnectionWrapper>> {
    let connection = ConnectionWrapper::open(path)?;

    Ok(unsafe { DB_CONNECTION.get_or_init(|| RwLock::new(connection)) })
}

pub fn disconnect() {
    let _ = unsafe { DB_CONNECTION.take() };
}

pub fn get_connection() -> &'static RwLock<ConnectionWrapper> {
    unsafe { DB_CONNECTION.get().unwrap() }
}
