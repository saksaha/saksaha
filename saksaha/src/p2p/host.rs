use crate::{
    node::socket::TcpSocket, pconfig::PersistedP2PConfig,
};
use log::{error, info};
use saksaha_p2p_discovery::Disc;
use saksaha_p2p_identity::Identity;
use saksaha_peer::peer_store::PeerStore;
use saksaha_task::task_queue::TaskQueue;
use std::sync::Arc;
use tokio::net::TcpListener;

use super::{
    dial_scheduler::DialScheduler,
    listener::Listener,
    ops::handshake::HandshakeOp,
    state::HostState,
    task::{Task, TaskRunner},
};

pub struct Host {
    disc: Arc<Disc>,
    dial_scheduler: Arc<DialScheduler>,
    task_queue: Arc<TaskQueue<Task>>,
}

impl Host {
    pub async fn init(
        p2p_config: PersistedP2PConfig,
        my_rpc_port: u16,
        p2p_socket: TcpSocket,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: String,
    ) -> Result<Host, String> {
        let identity = {
            let id = Identity::new(p2p_config.secret, p2p_config.public_key)?;
            Arc::new(id)
        };

        let peer_store = {
            let ps = PeerStore::new(10)?;
            Arc::new(ps)
        };

        let task_queue = {
            let q = TaskQueue::new("P2P".to_string(), Box::new(TaskRunner {}));
            Arc::new(q)
        };

        let host_state = {
            let s = HostState::new(
                identity.clone(),
                my_rpc_port,
                p2p_socket.port,
                task_queue.clone(),
                peer_store.clone(),
            );
            Arc::new(s)
        };

        let listener = {
            let l = Listener::new(p2p_socket.listener, host_state.clone());
            Arc::new(l)
        };

        let handshake_op = {
            let h = HandshakeOp::new(host_state.clone());
            Arc::new(h)
        };

        let disc = {
            let d = Disc::init(
                identity.clone(),
                disc_port,
                p2p_socket.port,
                bootstrap_urls,
                default_bootstrap_urls,
            )
            .await?;
            Arc::new(d)
        };

        let dial_scheduler = {
            let d = DialScheduler::new(
                disc.iter(),
                host_state.clone(),
                handshake_op.clone(),
            );
            Arc::new(d)
        };

        let host = Host {
            disc,
            dial_scheduler,
            task_queue,
        };

        Ok(host)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.disc.start().await?;

        self.dial_scheduler.start();

        self.task_queue.run_loop();

        // let disc_it = self.disc.iter();
        // let a = disc_it.next().await?;
        // println!("111,");

        // let handshake = Handshake::new(self.task_mng.clone());
        // let handshake_started = handshake.start(
        //     peer_store.clone(),
        //     Arc::new(disc_wakeup_tx),
        //     rpc_port,
        //     Arc::new(Mutex::new(peer_op_wakeup_rx)),
        //     credential_clone,
        //     peer_op_listener,
        // );

        // match handshake_started.await {
        //     handshake::Status::Launched => (),
        //     handshake::Status::SetupFailed(err) => {
        //         return HostStatus::SetupFailed(err);
        //     }
        // };

        Ok(())
    }
}
