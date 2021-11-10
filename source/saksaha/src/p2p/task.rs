use log::{debug, error, warn};
use p2p_identity::{PUBLIC_KEY_LEN, PeerId};
use p2p_transport::{TransportFactory, TransportInitError};
use peer::Peer;
use task::task_queue::{TaskResult, TaskRun};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) enum Task {
    InitiateHandshake {
        ip: String,
        p2p_port: u16,
        public_key: PeerId,
        transport_factory: Arc<TransportFactory>,
        peer: Arc<Peer>,
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
                    public_key,
                    transport_factory,
                    peer,
                } => {
                    match transport_factory.initiate_handshake(
                        ip,
                        p2p_port,
                        peer,
                    ).await {
                        Ok(_) => (),
                        Err(err) => {
                            handle_initiate_handshake_err(err);
                        }
                    };
                    // match handshake_op.initiate.send_handshake_syn(
                    //     ip,
                    //     p2p_port,
                    //     my_public_key,
                    // ).await {
                    //     Ok(_) => (),
                    //     Err(err) => {
                    //         let err_msg = err.to_string();

                    //         return TaskResult::FailRetriable(err_msg);
                    //     }
                    // };
                }
            };

            TaskResult::Success
        })
    }
}

fn handle_initiate_handshake_err(err: TransportInitError) {
    debug!("initiate handshake fail, err: {}", err);

    match err {
        TransportInitError::CallInProcess { .. } => (),
        TransportInitError::ConnectionFail { .. } => (),
        TransportInitError::MyEndpoint { .. } => (),
    };
}
