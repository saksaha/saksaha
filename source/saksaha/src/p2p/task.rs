use log::{debug, error, warn};
use p2p_active_calls::ActiveCalls;
use p2p_identity::{Identity, PeerId, PUBLIC_KEY_LEN};
use p2p_transport::{HandshakeInitParams, HandshakeInitError,};
use peer::Peer;
use tokio::sync::Mutex;
use std::sync::Arc;
use task::task_queue::{TaskResult, TaskRun};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateHandshake(HSInitTaskParams),
}

#[derive(Clone)]
pub(crate) struct HSInitTaskParams {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub her_ip: String,
    pub her_p2p_port: u16,
    pub her_public_key: PeerId,
    pub peer: Arc<Mutex<Peer>>,
    pub handshake_active_calls: Arc<ActiveCalls>,
}

pub(crate) struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                Task::InitiateHandshake(hs_init_task_params) => {
                    handle_initiate_handshake(hs_init_task_params).await;
                }
            };

            TaskResult::Success
        })
    }
}

async fn handle_initiate_handshake(hs_init_task_params: HSInitTaskParams) {
    let hs_init_params = HandshakeInitParams {
        identity: hs_init_task_params.identity,
        my_rpc_port: hs_init_task_params.my_rpc_port,
        my_p2p_port: hs_init_task_params.my_p2p_port,
        her_ip: hs_init_task_params.her_ip.clone(),
        her_p2p_port: hs_init_task_params.her_p2p_port,
        her_public_key: hs_init_task_params.her_public_key,
    };

    let peer = hs_init_task_params.peer;

    // let handshake_active_calls = hs_init_task_params.handshake_active_calls;

    // handshake_active_calls
    //     .insert_outbound(hs_init_task_params.her_ip.clone())
    //     .await;

    match p2p_transport::initiate_handshake(hs_init_params).await {
        Ok(t) => {

        },
        Err(err) => {
            debug!("initiate handshake fail, err: {}", err);

            match err {
                HandshakeInitError::CallInProcess { .. } => (),
                HandshakeInitError::ConnectionFail { .. } => (),
                HandshakeInitError::MyEndpoint { .. } => (),
                HandshakeInitError::PayloadWriteFail { .. } => (),
                HandshakeInitError::InvalidAck { .. } => (),
                HandshakeInitError::HandshakeSentFail { .. } => (),
                HandshakeInitError::Invalid { .. } => (),
            };
        }
    };

    // handshake_active_calls
    //     .remove(hs_init_task_params.her_ip)
    //     .await;
}
