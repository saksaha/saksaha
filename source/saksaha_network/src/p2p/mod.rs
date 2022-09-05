mod dial_scheduler;
mod host;
mod monitor;
mod server;
mod task;
mod testing;

#[cfg(test)]
mod tests;

pub(crate) use host::{P2PHost, P2PHostArgs};
pub(crate) use monitor::P2PMonitor;

pub(crate) type P2PHostError = Box<dyn std::error::Error + Send + Sync>;
