pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);

// Personal preference
pub use std::format as f;