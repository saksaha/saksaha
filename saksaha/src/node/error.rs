use thiserror::Error;

#[derive(Error, PartialEq, Debug)]
pub enum NodeError {
    #[error("Component init failed, cmp: {0}")]
    InitError(String),

    #[error("Component start failed, cmp: {0}")]
    StartFail(String),
}