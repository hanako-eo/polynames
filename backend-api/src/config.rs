use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}
