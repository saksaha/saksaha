use log::{debug, error, warn};
use saksaha_p2p_identity::PUBLIC_KEY_LEN;
use saksaha_task::task_queue::{TaskResult, TaskRun};
use std::{sync::Arc};

use super::ops::handshake::HandshakeOp;

#[derive(Clone)]
pub enum Task {
    SendHandshakeSyn {
        endpoint: String,
        my_public_key_bytes: [u8; PUBLIC_KEY_LEN],
        handshake_op: Arc<HandshakeOp>,
    }
}

pub struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                _ => (),
            };

            TaskResult::Success
        })
    }
}
