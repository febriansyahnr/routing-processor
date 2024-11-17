use crate::utils::envs;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic: {0}")]
    Generic(String),

    // -- Database
    #[error("Record not found in database")]
    RecordNotFound,

    #[error(transparent)]
    Envs(#[from] envs::Error),

    // -- External
    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SQLX(#[from] sqlx::Error),
}