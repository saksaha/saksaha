use log::{debug, error, warn};
use p2p_identity::{PeerId, PUBLIC_KEY_LEN};
use p2p_transport::{TransportFactory, TransportInitError};
use peer::Peer;
use std::sync::Arc;
use task::task_queue::{TaskResult, TaskRun};

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
                    handle_initiate_handshake(
                        ip,
                        p2p_port,
                        public_key,
                        transport_factory,
                        peer,
                    )
                    .await;
                }
            };

            TaskResult::Success
        })
    }
}

async fn handle_initiate_handshake(
    ip: String,
    p2p_port: u16,
    public_key: PeerId,
    transport_factory: Arc<TransportFactory>,
    peer: Arc<Peer>,
) {
    match transport_factory
        .initiate_handshake(ip, p2p_port, peer)
        .await
    {
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
