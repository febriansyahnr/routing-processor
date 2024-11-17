use core::{port::repository::TProcessor, prelude::*};
use crate::prelude::*;
use sqlx::mysql::MySqlPool;
use sqlx::types::chrono;
use core::repository::processor::ProcessorRepository;
use core::model;

pub struct ProcessorSeeder<'a> {
    pub name: &'static str,
    db: &'a MySqlPool
}

impl <'a> ProcessorSeeder<'a> {
    pub fn new(name: &'static str, db: &'a MySqlPool) -> Self {
        Self {
            name,
            db,
        }
    }
}

impl TSeeder for ProcessorSeeder<'_> {
    async fn execute(&self) -> Result<()> {
        println!("Seeding Processor {}", self.name);
        let now = chrono::Utc::now();

        let repo = ProcessorRepository::new(self.db);
        let processor_model = model::processor::Processor{
            base_url: "http://localhost:8080".to_string(),
            name: "test".to_string(),
            description: "test processor".to_string(),
            status: "active".to_string(),
            uuid: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
        };
        repo.create(&processor_model).await?;

        Ok(())
    }
}