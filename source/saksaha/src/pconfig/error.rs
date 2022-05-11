use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PConfigError {
    #[error("Init error, err: {err}")]
    InitError { err: String },

    #[error("Persist error, err: {err}")]
    PersistError { err: String },

    #[error("Read fail, err: {err}")]
    ReadFail { err: String },

    #[error("Serialization fail, err: {err}")]
    SerializationFail { err: String },

    #[error("Deserialization fail, err: {err}")]
    DeserializationFail { err: String },

    #[error("Path not found")]
    PathNotFound(PathBuf),

    #[error("Path creation fail, err: {err}")]
    PathCreationFail { err: String },

    #[error("Config write fail, err: {err}")]
    ConfigWriteFail { err: String },

    #[error("Unknown")]
    Unknown,
}
