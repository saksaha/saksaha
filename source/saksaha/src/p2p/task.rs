use log::{debug, error, warn};
use p2p_active_calls::ActiveCalls;
use p2p_identity::{Identity, PeerId, PUBLIC_KEY_LEN};
use p2p_transport::{HandshakeInitError, HandshakeInitParams};
use peer::{Peer, PeerStore, PeerValue, RegisteredPeerValue};
use std::{sync::Arc, pin::Pin};
use task::task_queue::{TaskResult, TaskRun};
use tokio::{sync::Mutex, io::AsyncBufReadExt, io::AsyncReadExt,};

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
    pub peer_store: Arc<PeerStore>,
    pub peer: Arc<Peer>,
    pub handshake_active_calls: Arc<ActiveCalls>,
}

pub(crate) struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    // fn run<'a>(
    //     &'a self,
    //     task: Task,
    // ) -> Box<dyn std::future::Future<Output = ()> + Send + 'a>
    // where
    //     Self: 'a,
    // {
    //     // async fn run(_self: &TaskRunner) {}

    //     // Box::pin(run(self))
    //     Box::new(async {})
    // }

    // fn run(&self, task: Task) -> TaskResult {
    //     futures::executor::block_on(async {
    //         match task {
    //             Task::InitiateHandshake(hs_init_task_params) => {
    //                 handle_initiate_handshake(hs_init_task_params).await;
    //             }
    //         };

    //         TaskResult::Success
    //     })
    // }
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

    let HSInitTaskParams {
        peer, peer_store, ..
    } = hs_init_task_params;

    // let handshake_active_calls = hs_init_task_params.handshake_active_calls;

    // handshake_active_calls
    //     .insert_outbound(hs_init_task_params.her_ip.clone())
    //     .await;

    match p2p_transport::initiate_handshake(hs_init_params).await {
        Ok(mut t) => {
            let mut p_val = peer.value.lock().await;
            *p_val =
                PeerValue::Registered(RegisteredPeerValue { transport: t });
            std::mem::drop(p_val);

            peer_store.register(peer.clone()).await;

            // tokio::spawn(async {
            //     loop {
            //         let mut buf = vec![];
            //         t.stream.read_buf(&mut buf);
            //     }
            // });
        }
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
