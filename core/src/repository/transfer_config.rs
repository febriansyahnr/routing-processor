use std::sync::Arc;
use sqlx::mysql::MySqlPool;
use crate::port::repository::TTransferConfig;
use crate::prelude::*;
use crate::model::transfer_config as model;

pub struct TransferConfigRepository {
    db: Arc<MySqlPool>
}

impl  TransferConfigRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        TransferConfigRepository {
            db
        }
    }
}

impl  TTransferConfig for TransferConfigRepository {
    async fn create(&self, config: &model::TransferConfig) -> Result<()> {
        sqlx::query(r#"
        insert into transfer_configs 
            (uuid, processor_uuid, priority, module, method, limit_config, status, created_at, updated_at) 
        values (?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
        .bind(config.uuid.to_string())
        .bind(config.processor_uuid.to_string())
        .bind(config.priority)
        .bind(config.module.to_owned())
        .bind(config.method.to_owned())
        .bind(config.limit_config.to_owned())
        .bind(config.status.to_owned())
        .bind(config.created_at)
        .bind(config.updated_at)
        .execute(self.db.as_ref())
        .await?;

        Ok(())
    }
    async fn get_transfer_config(&self, uuid: uuid::Uuid) -> Result<model::TransferConfigWithProcessor> {
        let query = r#"
            SELECT 
                tc.uuid,
                tc.priority,
                tc.module,
                tc.method,
                tc.limit_config,
                tc.status,
                p.uuid as processor_uuid, 
                p.name, 
                p.base_url, 
                p.description, 
                p.status, 
                p.created_at, 
                p.updated_at
            FROM transfer_configs tc
            LEFT JOIN processors p ON tc.processor_uuid = p.uuid
            WHERE tc.uuid = ?
            limit 1
        "#;
        let result: Option<model::TransferConfigWithProcessor> = sqlx::query_as(query)
            .bind(uuid)
            .fetch_optional(self.db.as_ref())
            .await?;
        let Some(config) = result else {
            return Err(Error::RecordNotFound);
        };
        Ok(config)
    }
    async fn get_transfer_config_routing(&self) -> Result<Vec<model::TransferConfigWithProcessor>> {
        let query = r#"
            SELECT 
                tc.uuid,
                tc.priority,
                tc.module,
                tc.method,
                tc.limit_config,
                tc.status,
                p.uuid as processor_uuid, 
                p.name, 
                p.base_url, 
                p.description, 
                p.status, 
                p.created_at, 
                p.updated_at
            FROM transfer_configs tc
            LEFT JOIN processors p ON tc.processor_uuid = p.uuid
            WHERE tc.status = 'active'
            AND p.status = 'active'
            ORDER BY tc.priority
        "#;

        let result: Vec<model::TransferConfigWithProcessor> = sqlx::query_as(query)
        .fetch_all(self.db.as_ref())
        .await?;

        Ok(result)
    }
}