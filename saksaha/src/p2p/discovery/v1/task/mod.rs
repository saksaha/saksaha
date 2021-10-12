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

// type BoxedFuture = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;
// type BoxedFuture =
//     Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>;

pub enum TaskResult<E> {
    Success,

    Retriable,

    Fail(E),
}

type Action = Pin<Box<dyn Future<Output = TaskResult<Error>> + Send + Sync>>;

pub struct Task {
    // action: Box<dyn Fn() -> Action + Send>,
    // pub kind:
    pub fail_count: usize,
}

impl Task {
    pub fn new<F>(action: F) -> Task
    where
        F: Future<Output = TaskResult<Error>> + 'static + Send + Sync,
    {
        Task {
            // action,
            fail_count: 0,
        }
    }
}

macro_rules! task {
    (async $d:tt) => {
        {
            let t = $crate::p2p::discovery::task::Task::new(
                || Box::pin(async $d));
            t
        }
    };

    (async move $d:tt) => {
        {
            let t = $crate::p2p::discovery::task::Task::new(
                || Box::pin(async move $d));
            t
        }
    };
}

pub(crate) use task;
