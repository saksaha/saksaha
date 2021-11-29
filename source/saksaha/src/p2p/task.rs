use log::{debug, error, warn};
use p2p_identity::{Identity, PeerId, PUBLIC_KEY_LEN};
use p2p_transport::{HandshakeArgs, TransportInitError};
use peer::Peer;
use std::sync::Arc;
use task::task_queue::{TaskResult, TaskRun};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateHandshake(HandshakeArgs),
}

pub(crate) struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                Task::InitiateHandshake(args) => {
                    handle_initiate_handshake(args).await;
                }
            };

            TaskResult::Success
        })
    }
}

async fn handle_initiate_handshake(handshake_args: HandshakeArgs) {
    match p2p_transport::initiate_handshake(handshake_args).await {
        Ok(_) => (),
        Err(err) => {
            debug!("initiate handshake fail, err: {}", err);

            match err {
                TransportInitError::CallInProcess { .. } => (),
                TransportInitError::ConnectionFail { .. } => (),
                TransportInitError::MyEndpoint { .. } => (),
            };
        }
    };
}
