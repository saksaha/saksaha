use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PConfigError {
    #[error("")]
    InitError,

    #[error("")]
    PersistError,

    #[error("")]
    ReadFail(String),

    #[error("")]
    SerializationFail(String),

    #[error("")]
    PathNotFound(PathBuf),

    #[error("")]
    PathCreationFail(String),

    #[error("")]
    ConfigWriteFail(String),

    #[error("")]
    Unknown,
}
