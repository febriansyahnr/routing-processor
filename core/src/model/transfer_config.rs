use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

use super::processor::Processor;

#[derive(Deserialize, Serialize, Clone)]
pub struct TransferConfigLimit {
    daily: i64,
}
impl TransferConfigLimit {
    pub fn new(daily: i64) -> Self {
        Self {
            daily,
        }
    }
}

impl Into<TransferConfigLimit> for Json<TransferConfigLimit> {
    fn into(self) -> TransferConfigLimit {
        self.0
    }
}


#[derive(Deserialize, Serialize, FromRow)]
pub struct TransferConfig {
    pub uuid: uuid::Uuid,
    pub processor_uuid: uuid::Uuid,
    pub priority: u64,
    pub module: String,
    pub method: String,
    pub limit_config: Json<TransferConfigLimit>,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl TransferConfig {
    pub fn new(
        processor_uuid: uuid::Uuid,
        priority: u64,
        module: String,
        method: String,
        limit_config: TransferConfigLimit,
    ) -> Self {
        let now = Utc::now();
        let uuid = uuid::Uuid::now_v7();
        let status = "active".to_string();
        Self {
            uuid,
            processor_uuid,
            priority,
            module,
            method,
            limit_config: limit_config.into(),
            status,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct TransferConfigWithProcessor {
    pub uuid: String,
    pub priority: i32,
    pub module: String,
    pub method: String,
    pub limit_config: Json<TransferConfigLimit>,
    pub status: String,
    pub processor_uuid: String,
    pub name: String,
    pub description: String,
    pub base_url: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}