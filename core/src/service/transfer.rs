use crate::model::transfer::TransferRequest;
use crate::prelude::*;
use crate::repository::transfer_config::TransferConfigRepository;
use crate::port::service;
use crate::port::repository::*;

pub struct TransferService <'a> {
    transfer_config_repo: Box<TransferConfigRepository<'a>>,
}

impl service::CanTransfer for TransferService<'_> {
    async fn transfer(&self, req: &TransferRequest) -> Result<()> {
        let routings = self.transfer_config_repo.get_transfer_config_routing().await?;

        for routing in routings {
            println!("Sending transfer with: {} - {} - {}", routing.name, routing.module, routing.method);
            println!("Transfer request: {:?}", req);
        }

        Ok(())
    }
}
