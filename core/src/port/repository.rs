use std::future::Future;
use uuid::Uuid;

use crate::model::processor::{Processor, ProcessorQuery};
use crate::prelude::*;

pub trait TProcessor {
    fn get_processor(&self, uuid: Uuid) -> impl Future<Output = Result<Processor>> + Send;
    fn get_all_processors(&self, query: ProcessorQuery) -> impl Future<Output = Result<Vec<Processor>>> + Send;
    fn create(&self, processor: &Processor) -> impl Future<Output = Result<()>> + Send;
    fn update(&self, processor: &Processor) -> impl Future<Output = Result<()>> + Send;
}