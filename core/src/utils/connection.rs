use std::time::Duration;
use crate::prelude::*;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

pub async fn get_mysql_pool(database_url: &str) -> Result<MySqlPool> {
    let db_url = database_url;
    let pool = MySqlPoolOptions::new()
    .max_connections(10)
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .idle_timeout(Duration::from_secs(60))
    .connect(db_url)
    .await?;

    Ok(pool)
}