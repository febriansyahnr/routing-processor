use std::future::Future;
use crate::model::processor::{Processor, ProcessorQuery};
use crate::prelude::*;

pub trait TProcessor {
    fn get_processor(&self, uuid: String) -> impl Future<Output = Result<Processor>> + Send;
    fn get_all_processors(&self, query: ProcessorQuery) -> impl Future<Output = Result<Vec<Processor>>> + Send;
}