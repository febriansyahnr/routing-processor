pub use crate::error::Error;
use crate::service::transfer::TransferService;
pub type Result<T> = std::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub transfer_serice: Arc<TransferService>,
}