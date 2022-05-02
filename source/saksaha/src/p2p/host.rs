use super::server::Server;
use super::state::HostState;
use crate::network::socket::TcpSocket;
use colored::Colorize;
use logger::tinfo;
use p2p_active_calls::ActiveCalls;
use p2p_discovery::{Discovery, DiscoveryArgs};
use p2p_identity::{addr::Addr, identity::P2PIdentity, peer::UnknownPeer};
use peer::PeerStore;
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::TcpListener;

pub(crate) struct Host {
    pub(crate) host_state: Arc<HostState>,
    discovery: Arc<Discovery>,
    // dial_scheduler: Arc<DialScheduler>,
    // task_queue: Arc<TaskQueue<Task>>,
    server: Arc<Server>,
}

pub(crate) struct HostArgs {
    pub(crate) p2p_socket: Arc<TcpListener>,
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) disc_task_interval: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) p2p_port: u16,
    pub(crate) disc_port: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<Addr>,
    pub(crate) rpc_port: u16,
    pub(crate) secret: String,
    pub(crate) public_key: String,
    pub(crate) peer_store: Arc<PeerStore>,
}

impl Host {
    pub async fn init(host_args: HostArgs) -> Result<Host, String> {
        let p2p_identity = {
            let id = P2PIdentity::new(host_args.secret, host_args.public_key)?;
            Arc::new(id)
        };

        tinfo!(
            "saksaha",
            "p2p",
            "Created p2p identity, public_key: {}",
            p2p_identity.public_key.yellow(),
        );

        // let task_queue = {
        //     let q = TaskQueue::new(10);
        //     Arc::new(q)
        // };

        let handshake_active_calls = {
            let a = ActiveCalls::new();
            Arc::new(a)
        };

        let host_state = {
            // let s = HostState::new(
            //     p2p_identity.clone(),
            //     host_args.rpc_port,
            //     host_args.p2p_port,
            //     // task_queue.clone(),
            //     host_args.peer_store.clone(),
            //     handshake_active_calls,
            // );
            let s = HostState {
                p2p_identity: p2p_identity.clone(),
                my_rpc_port: host_args.rpc_port,
                my_p2p_port: host_args.p2p_port,
                peer_store: host_args.peer_store.clone(),
                handshake_active_calls,
            };
            Arc::new(s)
        };

        let server = {
            let s = Server::new(host_args.p2p_socket, host_state.clone());
            Arc::new(s)
        };

        let disc_args = DiscoveryArgs {
            disc_dial_interval: host_args.disc_dial_interval,
            disc_table_capacity: host_args.disc_table_capacity,
            disc_task_interval: host_args.disc_task_interval,
            p2p_identity: p2p_identity.clone(),
            disc_port: host_args.disc_port,
            p2p_port: host_args.p2p_port,
            bootstrap_addrs: host_args.bootstrap_addrs,
        };

        let discovery = {
            let d = Discovery::init(disc_args).await?;
            Arc::new(d)
        };

        // let dial_scheduler = {
        //     let d = DialScheduler::new(
        //         discovery.iter(),
        //         host_state.clone(),
        //         host_args.p2p_dial_interval.clone(),
        //     );
        //     Arc::new(d)
        // };

        let host = Host {
            discovery,
            // dial_scheduler,
            // task_queue,
            server,
            host_state,
        };

        Ok(host)
    }

    pub async fn start(&self) -> Result<(), String> {
        let local_addr = match self.server.tcp_socket.local_addr() {
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
