use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PConfigError {
    #[error("Init error")]
    InitError,

    #[error("Persist error")]
    PersistError,

    #[error("Read fail")]
    ReadFail(String),

    #[error("Serialization fail")]
    SerializationFail(String),

    #[error("Deserialization fail")]
    DeserializationFail(String),

    #[error("Path not found")]
    PathNotFound(PathBuf),

    #[error("Path creation fail")]
    PathCreationFail(String),

    #[error("Config write fail")]
    ConfigWriteFail(String),

    #[error("Unknown")]
    Unknown,
}
