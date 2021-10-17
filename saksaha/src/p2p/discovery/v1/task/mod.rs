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

struct Task {
    kind: TaskKind,
    fail_count: usize,
}

pub enum TaskKind {
    InitiateWhoAreYou(),
    // pub action: Box<dyn Fn()>,
    // // action: Box<dyn Fn() -> Action + Send>,
    // // pub kind:
    // pub fail_count: usize,
}

// impl Task {
//     pub fn new(action: Box<dyn Fn()>) -> Task {
//         Task {
//             action,
//             fail_count: 0,
//         }
//     }
// }

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
