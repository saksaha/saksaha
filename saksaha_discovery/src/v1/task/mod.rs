use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub enum TaskResult {
    Success,
    Retriable,
}

pub enum TaskError {
    Default(String),
}

// type Action = Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>;

struct Task {
    kind: TaskKind,
    fail_count: usize,
}

pub enum TaskKind {
    // Ping(Address),
}

