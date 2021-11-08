use crate::{
    node::socket::TcpSocket, pconfig::PersistedP2PConfig,
};
use log::{error, info};
use saksaha_p2p_discovery::Disc;
use saksaha_p2p_identity::Identity;
use saksaha_p2p_transport::TransportFactory;
use saksaha_peer::PeerStore;
use saksaha_task::task_queue::TaskQueue;
use std::sync::Arc;
use tokio::net::TcpListener;
use super::{
    dial_scheduler::DialScheduler,
    listener::Listener,
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
            let ps = PeerStore::init().await?;
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

        let transport_factory = {
            let f = TransportFactory::new(
                identity.clone(),
                host_state.my_rpc_port,
                host_state.my_p2p_port,
            );
            Arc::new(f)
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
                transport_factory.clone(),
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

        Ok(())
    }
}
