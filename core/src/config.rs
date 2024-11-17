use std::sync::OnceLock;
use crate::prelude::*;
use crate::utils::envs::get_env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        dotenvy::dotenv().ok();
        Ok(Config {
            database_url: get_env("DATABASE_URL")?,
        })
    }
    
}

pub fn get_config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::new().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}