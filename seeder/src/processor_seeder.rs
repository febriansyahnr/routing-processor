use core::{port::repository::TProcessor, prelude::*};
use std::sync::Arc;
use crate::prelude::*;
use sqlx::mysql::MySqlPool;
use core::repository::processor::ProcessorRepository;
use core::model;

pub struct ProcessorSeeder {
    pub name: &'static str,
    db: Arc<MySqlPool>
}

impl ProcessorSeeder {
    pub fn new(name: &'static str, db: Arc<MySqlPool>) -> Self {
        Self {
            name,
            db,
        }
    }
}

impl TSeeder for ProcessorSeeder {
    async fn execute(&self) -> Result<()> {
        println!("Seeding Processor {}", self.name);

        let repo = ProcessorRepository::new(self.db.clone());
        let processor_model = model::processor::Processor
            ::new(
                "snap-core-processor", 
                "http://localhost", 
                "snap core",
            );
        repo.create(&processor_model).await?;

        Ok(())
    }
}