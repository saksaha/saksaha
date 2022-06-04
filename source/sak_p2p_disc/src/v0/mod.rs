mod addr_monitor_routine;
mod dial_scheduler;
mod discovery;
mod net;
mod ops;
mod server;
mod table;
mod task;

#[cfg(test)]
mod tests;

pub use discovery::{Discovery, DiscoveryArgs};
pub(crate) use net::*;
pub(crate) use ops::*;
pub use table::*;

pub(crate) type BoxedError = Box<dyn std::error::Error + Send + Sync>;
