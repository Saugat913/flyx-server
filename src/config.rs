/// This is the configuration handler for loading the configuration for the server
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            server_address: std::env::var("SERVER_ADDRESS").unwrap(),
        }
    }
}
