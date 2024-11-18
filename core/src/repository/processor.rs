use std::sync::Arc;

use sqlx::mysql::MySqlPool;
use crate::model::processor::{Processor, ProcessorQuery};
use crate::port::repository::TProcessor;
use crate::prelude::*;

pub struct ProcessorRepository {
    db: Arc<MySqlPool>
}

impl  ProcessorRepository {
    pub fn new(db: Arc<MySqlPool>) -> Self {
        ProcessorRepository {
            db
        }
    }
}

impl TProcessor for ProcessorRepository {
    async fn get_processor(&self, uuid: uuid::Uuid) -> Result<Processor> {
        let result: Processor = sqlx::query_as("select * from processors where uuid = ?")
            .bind(uuid)
            .fetch_one(self.db.as_ref())
            .await?;

        Ok(result)
    }
    async fn get_all_processors(&self, q: ProcessorQuery) -> Result<Vec<Processor>> {
        let (where_clause, values) = q.get_query();

        let sql_query = format!("select * from processors {}", where_clause);
        let mut sql_query = sqlx::query_as(&sql_query);
        if values.is_empty() {
            return Err(Error::RecordNotFound);
        }

        for val in values {
            sql_query = sql_query.bind(val);
        }

        let result: Vec<Processor> = sql_query
            .fetch_all(self.db.as_ref())
            .await?;

        Ok(result)
    }
    async fn create(&self, processor: &Processor) -> Result<()> {
        sqlx::query("insert into processors (uuid, name, base_url, description, status, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?)")
        .bind(processor.uuid.to_string())
        .bind(processor.name.to_owned())
        .bind(processor.base_url.to_owned())
        .bind(processor.description.to_owned())
        .bind(processor.status.to_owned())
        .bind(processor.created_at)
        .bind(processor.updated_at)
        .execute(self.db.as_ref())
        .await?;
        Ok(())
    }
    async fn update(&self, processor: &Processor) -> Result<()> {
        sqlx::query("update processors set name = ?, description = ?, status = ?, updated_at = ? where uuid = ?")
        .bind(processor.name.to_owned())
        .bind(processor.description.to_owned())
        .bind(processor.status.to_owned())
        .bind(processor.updated_at)
        .bind(processor.uuid.to_owned())
        .execute(self.db.as_ref())
        .await?;
        Ok(())
    }
}

mod test_processor_repo {
    use super::*;

    #[tokio::test]
    async fn test_connection() -> Result<()> {
        let config = crate::config::Config::new()?;
        let pool = crate::utils::connection::get_mysql_pool(&config.database_url).await?;
        let repo = ProcessorRepository::new(pool);
        let result = repo.get_all_processors(ProcessorQuery::default()).await?;
        let mut last_uuid = uuid::Uuid::nil();

        for processor in result {
            println!("{:#?}", processor);
            last_uuid = processor.uuid;
        }

        let last_processor = repo.get_processor(last_uuid).await?;
        println!("Last Processor:\n\t{:#?}", last_processor);
        assert_eq!(last_processor.uuid, last_uuid);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_processors() -> Result<()> {
        let config = crate::config::Config::new()?;
        let pool = crate::utils::connection::get_mysql_pool(&config.database_url).await?;
        let repo = ProcessorRepository::new(pool);
        let result = repo.get_all_processors(ProcessorQuery::new(None, Some("active".to_owned()))).await?;
        for processor in result {
            println!("{:#?}", processor);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_processors_with_query_name() -> Result<()> {
        let  config = crate::config::Config::new()?;
        let pool = crate::utils::connection::get_mysql_pool(&config.database_url).await?;
        let repo = ProcessorRepository::new(pool);
        let result = repo.get_all_processors(ProcessorQuery::new(Some("snap-core-processor".to_owned()), None)).await?;
        for processor in &result {
            println!("{:#?}", processor);
        }
        assert_eq!(result.len(), 1);
        Ok(())
    }
}