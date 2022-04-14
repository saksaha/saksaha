use super::{
    dial_scheduler::DialScheduler,
    identity::Identity,
    listener::Listener,
    state::HostState,
    task::{P2PTaskHandler, Task},
};
use crate::network::socket::TcpSocket;
use colored::Colorize;
use logger::tinfo;
use p2p_active_calls::ActiveCalls;
use p2p_discovery::{Discovery, DiscoveryArgs};
use p2p_identity::{peer::UnknownPeer, P2PIdentity};
use peer::PeerStore;
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::TcpListener;

pub(crate) struct Host {
    pub host_state: Arc<HostState>,
    discovery: Arc<Discovery>,
    dial_scheduler: Arc<DialScheduler>,
    task_queue: Arc<TaskQueue<Task>>,
    listener: Arc<Listener>,
}

pub(crate) struct HostArgs {
    pub(crate) p2p_socket: Arc<TcpListener>,
    pub(crate) p2p_port: u16,
    pub(crate) disc_port: Option<u16>,
    pub(crate) unknown_peers: Vec<UnknownPeer>,
    pub(crate) rpc_port: u16,
    pub(crate) identity: Identity,
    pub(crate) bootstrap_urls: Option<Vec<String>>,
    pub(crate) peer_store: Arc<PeerStore>,
}

impl Host {
    pub async fn init(host_args: HostArgs) -> Result<Host, String> {
        let identity = {
            let id = P2PIdentity::new(
                host_args.identity.secret,
                host_args.identity.public_key,
            )?;
            Arc::new(id)
        };

        let task_queue = {
            let q = TaskQueue::new(10);
            Arc::new(q)
        };

        let handshake_active_calls = {
            let a = ActiveCalls::new();
            Arc::new(a)
        };

        let host_state = {
            let s = HostState::new(
                identity.clone(),
                host_args.rpc_port,
                host_args.p2p_port,
                task_queue.clone(),
                host_args.peer_store.clone(),
                handshake_active_calls,
            );
            Arc::new(s)
        };

        let listener = {
            let l = Listener::new(host_args.p2p_socket, host_state.clone());
            Arc::new(l)
        };

        let disc_args = DiscoveryArgs {
            p2p_identity: identity.clone(),
            disc_port: host_args.disc_port,
            p2p_port: host_args.p2p_port,
            bootstrap_urls: host_args.bootstrap_urls,
            unknown_peers: host_args.unknown_peers,
        };

        let discovery = {
            let d = Discovery::init(disc_args).await?;
            Arc::new(d)
        };

        let dial_scheduler = {
            let d = DialScheduler::new(discovery.iter(), host_state.clone());
            Arc::new(d)
        };

        let host = Host {
            discovery,
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

        self.discovery.start().await?;

        // self.listener.start();

        // self.dial_scheduler.start();

        // self.task_queue.run_loop();

        Ok(())
    }
}
