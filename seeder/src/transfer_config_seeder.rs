use sqlx::MySqlPool;
use crate::prelude::TSeeder;
use core::{model::{processor::Processor, transfer_config::{TransferConfig, TransferConfigLimit}}, port::repository::{TProcessor, TTransferConfig}, prelude::*, repository::{processor::ProcessorRepository, transfer_config::TransferConfigRepository}};

pub struct TransferConfigSeeder<'a> {
    pub name: &'static str,
    db: &'a MySqlPool
}

impl <'a> TransferConfigSeeder<'a> {
    pub fn new(name: &'static str, db: &'a MySqlPool) -> Self {
        Self {
            name,
            db,
        }
    }
}

impl TSeeder for TransferConfigSeeder<'_> {
    async fn execute(&self) -> Result<()> {
        println!("Seeding TransferConfig {}", self.name);
        let processor_repo = ProcessorRepository::new(self.db);
        let snap_core_processor = Processor::new(
            "snap-core-processor",
            "http://localhost",
            "snap core",
        );
        processor_repo.create(&snap_core_processor).await?;

        let transfer_config_repo = TransferConfigRepository::new(self.db);
        let modules: [String;3] = [
            "BCA".to_string(),
            "BRI".to_string(),
            "MANDIRI".to_string(),
        ];
        let mut index = 1;
        for modules in modules {
            let transfer_config = TransferConfig::new(
                snap_core_processor.uuid, 
                index, 
                modules.to_owned(), 
                "INTRABANK".to_string(), 
                TransferConfigLimit::new(5_000_000_000),
            );
            index += 1;
            transfer_config_repo.create(&transfer_config).await?;
            let transfer_config = TransferConfig::new(
                snap_core_processor.uuid, 
                index, 
                modules.to_owned(), 
                "BIFAST".to_string(), 
                TransferConfigLimit::new(500_000_000),
            );
            index += 1;
            transfer_config_repo.create(&transfer_config).await?;
            let transfer_config = TransferConfig::new(
                snap_core_processor.uuid, 
                index, 
                modules.to_owned(), 
                "INTERBANK".to_string(), 
                TransferConfigLimit::new(250_000_000),
            );
            transfer_config_repo.create(&transfer_config).await?;
            index += 1;
        }
        

        Ok(())
    }
}