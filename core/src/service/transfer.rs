use std::collections::HashMap;
use crate::model::transfer::TransferResponse;
use crate::model::transfer as model;
use crate::model::transfer::TransferRequest;
use crate::prelude::*;
use crate::port::service::*;
use crate::port::repository::*;
use crate::repository::transfer_config::TransferConfigRepository;

pub struct TransferService  {
    transfer_config_repo: Box<TransferConfigRepository>,
    core_processors: HashMap<String, Box<EProcessors>>,
}

impl TransferService {
    pub fn new(
        transfer_config_repo: Box<TransferConfigRepository>, 
        core_processors: HashMap<String, Box<EProcessors>>) -> Self {
        Self { transfer_config_repo, core_processors }
    }
}

impl CanTransfer for TransferService {
    async fn transfer(&self, req: &TransferRequest) -> Result<TransferResponse> {
        let routings = self.transfer_config_repo.get_transfer_config_routing().await?;
        let mut resp = TransferResponse::new(
            "Transaction in Progress", 
            model::ETransferStatus::Pending { code: "".to_string() }
        );

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
            match result.status {
                model::ETransferStatus::Pending{ code: _ } => {
                    return Ok(result)
                },
                model::ETransferStatus::Success => {
                    return Ok(result)
                },
                _ => {            
                    resp = result.clone();
                    continue
                },
            }
        }

        Ok(resp)
    }
}
