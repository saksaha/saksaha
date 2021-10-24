use thiserror::Error;

#[derive(Error, Debug)]
pub enum P2PError {
    #[error("Error setting up p2p")]
    SetupFail(String),
}
