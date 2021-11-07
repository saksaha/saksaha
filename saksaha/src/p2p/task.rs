use log::{debug, error, warn};
use saksaha_p2p_identity::PUBLIC_KEY_LEN;
use saksaha_task::task_queue::{TaskResult, TaskRun};
use std::sync::Arc;

use super::ops::handshake::HandshakeOp;

#[derive(Clone)]
pub(crate) enum Task {
    InitiateHandshake {
        ip: String,
        p2p_port: u16,
        my_public_key: [u8; PUBLIC_KEY_LEN],
        handshake_op: Arc<HandshakeOp>,
    },
}

pub(crate) struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                Task::InitiateHandshake {
                    ip,
                    p2p_port,
                    my_public_key,
                    handshake_op,
                } => {
                    match handshake_op.initiate.send_handshake_syn(
                        ip,
                        p2p_port,
                        my_public_key,
                    ).await {
                        Ok(_) => (),
                        Err(err) => {
                            let err_msg = err.to_string();

                            return TaskResult::FailRetriable(err_msg);
                        }
                    };
                }
            };

            TaskResult::Success
        })
    }
}
