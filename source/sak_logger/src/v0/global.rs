use crate::SakLogger;
use once_cell::sync::{Lazy, OnceCell};
use std::collections::HashMap;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

pub static LOGGER: OnceCell<SakLogger> = OnceCell::new();

// pub static mut NON_BLOCKINGS: Lazy<HashMap<String, (NonBlocking, WorkerGuard)>> =
//     Lazy::new(|| HashMap::new());
