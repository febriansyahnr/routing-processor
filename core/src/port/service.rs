use std::future::Future;
use crate::model::transfer::{self as model, TransferResponse};
use crate::{
    prelude::*,
    model::transfer::TransferRequest, 
};

pub trait CanTransfer {
    fn transfer(&self, req: &TransferRequest) -> impl Future<Output = Result<TransferResponse>> + Send;
}

use crate::repository::processors::{
    snap_core_processor::SnapCoreProcessor, 
    xb_core_processor::XBCoreProcessor,
};

use super::repository::TCoreProcessor;

pub enum EProcessors {
    SnapCore(SnapCoreProcessor),
    XBCore(XBCoreProcessor)
}

impl TCoreProcessor for EProcessors {
    async fn transfer(&self, req: &TransferRequest, method: model::ETransferMethod) -> Result<model::TransferResponse> {
        match self {
            Self::SnapCore(processor) => processor.transfer(req, method.to_owned()).await,
            Self::XBCore(processor) => processor.transfer(req, method.to_owned()).await,
        }
    }
}