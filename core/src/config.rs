use crate::prelude::*;
use crate::utils::envs::get_env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        dotenvy::dotenv().ok();
        let database_url = get_env("DATABASE_URL")?;
        Ok(Config {
            database_url,
        })
    }
    
}