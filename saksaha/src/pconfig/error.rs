use std::path::PathBuf;

pub enum PConfigError {
    InitError,

    PersistError,

    ReadFail(String),

    SerializationFail(String),

    PathNotFound(PathBuf),

    PathCreationFail(String),

    ConfigWriteFail(String),

    Unknown,
}
