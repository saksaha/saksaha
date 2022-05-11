use crate::ActiveCalls;
use logger::terr;
use std::sync::Arc;

#[derive(Debug)]
pub enum Call {
    Inbound { endpoint: String },
    Outbound { endpoint: String },
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inbound { endpoint } => {
                write!(f, "Inbound call from - {}", endpoint)
            }
            Self::Outbound { endpoint } => {
                write!(f, "Outbound call to - {}", endpoint)
            }
        }
    }
}

pub struct CallGuard {
    pub endpoint: String,
    pub active_calls: Arc<ActiveCalls>,
}

impl Drop for CallGuard {
    fn drop(&mut self) {
        match self.active_calls.delayed_remove(self.endpoint.clone()) {
            Ok(_) => (),
            Err(err) => {
                terr!(
                    "p2p_active_calls",
                    "",
                    "Call removal error, err: {}",
                    err
                );
            }
        }
    }
}
