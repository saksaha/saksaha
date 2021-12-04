use log::{debug, error, warn};
use p2p_active_calls::ActiveCalls;
use p2p_identity::{Identity, PeerId, PUBLIC_KEY_LEN};
use p2p_transport::{HandshakeArgs, TransportInitError};
use peer::Peer;
use std::sync::Arc;
use task::task_queue::{TaskResult, TaskRun};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateHandshake(InitHandshakeArgs),
}

#[derive(Clone)]
pub(crate) struct InitHandshakeArgs {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub her_ip: String,
    pub her_p2p_port: u16,
    pub her_public_key: PeerId,
    pub peer: Arc<Peer>,
    pub handshake_active_calls: Arc<ActiveCalls>,
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

async fn handle_initiate_handshake(init_handshake_args: InitHandshakeArgs) {
    let handshake_args = HandshakeArgs {
        identity: init_handshake_args.identity,
        my_rpc_port: init_handshake_args.my_rpc_port,
        my_p2p_port: init_handshake_args.my_p2p_port,
        her_ip: init_handshake_args.her_ip.clone(),
        her_p2p_port: init_handshake_args.her_p2p_port,
        her_public_key: init_handshake_args.her_public_key,
    };

    let handshake_active_calls = init_handshake_args.handshake_active_calls;

    handshake_active_calls
        .insert_outbound(init_handshake_args.her_ip.clone())
        .await;

    match p2p_transport::initiate_handshake(handshake_args).await {
        Ok(_) => (),
        Err(err) => {
            debug!("initiate handshake fail, err: {}", err);

            match err {
                TransportInitError::CallInProcess { .. } => (),
                TransportInitError::ConnectionFail { .. } => (),
                TransportInitError::MyEndpoint { .. } => (),
                TransportInitError::PayloadWriteFail { .. } => (),
                TransportInitError::InvalidAck { .. } => (),
                TransportInitError::HandshakeSentFail { .. } => (),
            };
        }
    };

    handshake_active_calls
        .remove(init_handshake_args.her_ip)
        .await;
}
