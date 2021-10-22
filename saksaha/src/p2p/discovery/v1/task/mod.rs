pub mod queue;

use crate::{
    common::{Error, Result},
    err,
};
use logger::log;
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use super::address::Address;

// type BoxedFuture = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;
// type BoxedFuture =
//     Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>;

pub enum TaskResult<E> {
    Success,

    Retriable,

    Fail(E),
}

type Action = Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>;

struct Task {
    kind: TaskKind,
    fail_count: usize,
}

pub enum TaskKind {
    Ping(Address),
}

