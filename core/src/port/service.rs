use std::future::Future;
use crate::{
    prelude::*,
    model::transfer::TransferRequest, 
};

pub trait TTransfer {
    fn transfer(&self, req: &TransferRequest) -> impl Future<Output = Result<()>> + Send;
}