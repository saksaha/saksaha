use super::{
    dial_scheduler::DialScheduler,
    listener::Listener,
    state::HostState,
    task::{P2PTaskRunner, Task},
};
use crate::{network::socket::TcpSocket, pconfig::PersistedP2PConfig};
use colored::Colorize;
use logger::tinfo;
use p2p_active_calls::ActiveCalls;
use p2p_discovery::Discovery;
use p2p_identity::Identity;
use peer::PeerStore;
use std::sync::Arc;
use task::task_queue::TaskQueue;

pub(crate) struct Host {
    pub host_state: Arc<HostState>,
    disc: Arc<Discovery>,
    dial_scheduler: Arc<DialScheduler>,
    task_queue: Arc<TaskQueue<Task>>,
    listener: Arc<Listener>,
}

impl Host {
    pub async fn init(
        p2p_config: PersistedP2PConfig,
        my_rpc_port: u16,
        p2p_socket: TcpSocket,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        // default_bootstrap_urls: &str,
    ) -> Result<Host, String> {
        let identity = {
            let id = Identity::new(
                p2p_config.identity.secret,
                p2p_config.identity.public_key,
            )?;
            Arc::new(id)
        };

        let peer_store = {
            let ps = PeerStore::init().await?;
            Arc::new(ps)
        };

        let task_queue = {
            let q =
                TaskQueue::new("p2p".to_string(), Box::new(P2PTaskRunner {}));
            Arc::new(q)
        };

        let handshake_active_calls = {
            let a = ActiveCalls::new();
            Arc::new(a)
        };

        let host_state = {
            let s = HostState::new(
                identity.clone(),
                my_rpc_port,
                p2p_socket.port,
                task_queue.clone(),
                peer_store.clone(),
                handshake_active_calls,
            );
            Arc::new(s)
        };

        let listener = {
            let l = Listener::new(p2p_socket.listener, host_state.clone());
            Arc::new(l)
        };

        let disc = {
            let d = Discovery::init(
                identity.clone(),
                disc_port,
                p2p_socket.port,
                bootstrap_urls,
                // default_bootstrap_urls,
            )
            .await?;
            Arc::new(d)
        };

        let dial_scheduler = {
            let d = DialScheduler::new(disc.iter(), host_state.clone());
            Arc::new(d)
        };

        let host = Host {
            disc,
            dial_scheduler,
            task_queue,
            listener,
            host_state,
        };

        Ok(host)
    }

    pub async fn start(&self) -> Result<(), String> {
        let local_addr = match self.listener.tcp_socket.local_addr() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "Couldn't get the local addr of tcp socket, err: {}",
                    err,
                ))
            }
        };

        tinfo!(
            "saksaha",
            "p2p",
            "p2p host is starting, tcp socket: {}",
            local_addr.to_string().yellow(),
        );

        self.disc.start().await?;

        // self.listener.start();

        // self.dial_scheduler.start();

        // self.task_queue.run_loop();

        Ok(())
    }
}
