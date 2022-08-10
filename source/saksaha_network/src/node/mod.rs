mod event_handle;
mod local_node;
mod miner;
mod msg_handle;
mod peer_node;
mod task;

#[cfg(test)]
mod tests;

pub(crate) use local_node::*;

pub(crate) type SaksahaNodeError = Box<dyn std::error::Error + Send + Sync>;
