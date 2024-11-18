use std::collections::HashMap;
use crate::model::transfer as model;
use crate::model::transfer::TransferRequest;
use crate::prelude::*;
use crate::port::service::*;
use crate::port::repository::*;
use crate::repository::transfer_config::TransferConfigRepository;

pub struct TransferService <'a> {
    transfer_config_repo: Box<TransferConfigRepository<'a>>,
    core_processors: HashMap<String, Box<EProcessors>>,
}

impl CanTransfer for TransferService<'_> {
    async fn transfer(&self, req: &TransferRequest) -> Result<()> {
        let routings = self.transfer_config_repo.get_transfer_config_routing().await?;

        for routing in routings {
            println!("Sending transfer with: {} - {} - {}", routing.name, routing.module, routing.method);
            println!("Transfer request: {:?}", req);
            let transfer_method: model::ETransferMethod = routing.method.into();
            let processor = self.core_processors
                .get(&routing.name)
                .take();
            let result = match processor {
                Some(processor) => processor.transfer(req, transfer_method).await?,
                None => continue,
            };

            println!("Transfer response: {:?}", result);
        }

        Ok(())
    }
}
