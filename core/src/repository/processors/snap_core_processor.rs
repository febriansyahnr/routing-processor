use crate::port::repository::TCoreProcessor;
use crate::model::transfer as model;
use crate::prelude::*;

pub struct SnapCoreProcessor{}

impl SnapCoreProcessor {
    pub fn new() -> Self {
        SnapCoreProcessor{}
    }
}

impl TCoreProcessor for SnapCoreProcessor {
    async  fn transfer(&self, req: &model::TransferRequest, _: model::ETransferMethod) -> Result<model::TransferResponse> {
        match req.beneficiary_account.as_str() {
            "99910001" => {
                return Ok(model::TransferResponse::new("Transaction Failed", model::ETransferStatus::Failed));
            },
            "99910002" => {
                return Ok(model::TransferResponse::new(
                    "Transaction Pending", 
                    model::ETransferStatus::Pending{ 
                        code: "504xx00".to_string() 
                    },
                ));
            },
            _ => {},
        }
        
        Ok(model::TransferResponse::default())
    }
}