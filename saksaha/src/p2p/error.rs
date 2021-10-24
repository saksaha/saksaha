use thiserror::Error;

#[derive(Error, Debug)]
pub enum HostError {
    #[error("Error setting up p2p, {0}")]
    SetupFail(String),
}
