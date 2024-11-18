use std::future::Future;
use uuid::Uuid;

use crate::model::processor::{Processor, ProcessorQuery};
use crate::model::transfer::{ETransferMethod, TransferRequest, TransferResponse};
use crate::model::transfer_config::{TransferConfig, TransferConfigWithProcessor};
use crate::prelude::*;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait TProcessor {
    fn get_processor(&self, uuid: Uuid) -> impl Future<Output = Result<Processor>> + Send;
    fn get_all_processors(&self, query: ProcessorQuery) -> impl Future<Output = Result<Vec<Processor>>> + Send;
    fn create(&self, processor: &Processor) -> impl Future<Output = Result<()>> + Send;
    fn update(&self, processor: &Processor) -> impl Future<Output = Result<()>> + Send;
}

#[cfg_attr(test, automock)]
pub trait TTransferConfig {
    fn get_transfer_config(&self, uuid: Uuid) -> impl Future<Output = Result<TransferConfigWithProcessor>> + Send;
    fn get_transfer_config_routing(&self) -> impl Future<Output = Result<Vec<TransferConfigWithProcessor>>> + Send;
    fn create(&self, config: &TransferConfig) -> impl Future<Output = Result<()>> + Send;
}

#[cfg_attr(test, automock)]
pub trait TCoreProcessor {
    fn transfer(&self, req: &TransferRequest, method: ETransferMethod) -> impl Future<Output = Result<TransferResponse>> + Send;
}