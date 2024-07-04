use std::ops::Deref;
use std::path::Path;

use duckdb::{Connection, Result};

pub struct ConnectionWrapper(Connection);

impl ConnectionWrapper {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Self(Connection::open(path)?))
    }
}

unsafe impl Sync for ConnectionWrapper {}

impl Deref for ConnectionWrapper {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Flatten<T, E> {
    fn flatten(self) -> Result<T, E>;
}

impl<T, E> Flatten<T, E> for Result<Result<T, E>, E> {
    #[inline]
    fn flatten(self) -> Result<T, E> {
        match self {
            Ok(inner) => inner,
            Err(err) => Err(err),
        }
    }
}
