use core::{port::repository::TProcessor, prelude::*};
use crate::prelude::*;
use sqlx::mysql::MySqlPool;
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

        let repo = ProcessorRepository::new(self.db);
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