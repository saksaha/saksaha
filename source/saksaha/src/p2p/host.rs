use super::dial_scheduler::P2PDialSchedulerArgs;
use super::task::runtime::P2PTaskRuntime;
use super::task::P2PTask;
use super::P2PMonitor;
use super::{dial_scheduler::P2PDialScheduler, server::Server};
use colored::Colorize;
use logger::tinfo;
use p2p_addr::UnknownAddr;
use p2p_discovery::{Discovery, DiscoveryArgs};
use p2p_identity::{Credential, Identity};
use p2p_peer::PeerTable;
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::TcpListener;

const P2P_TASK_QUEUE_CAPACITY: usize = 10;

pub(crate) struct P2PHost {
    p2p_discovery: Arc<Discovery>,
    p2p_dial_scheduler: P2PDialScheduler,
    p2p_server: Server,
    p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    p2p_task_runtime: P2PTaskRuntime,
    peer_table: Arc<PeerTable>,
}

pub(crate) struct P2PHostArgs {
    pub(crate) addr_expire_duration: Option<i64>,
    pub(crate) disc_port: Option<u16>,
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) disc_task_interval: Option<u16>,
    pub(crate) disc_task_queue_capacity: Option<u16>,
    pub(crate) p2p_socket: TcpListener,
    pub(crate) p2p_task_interval: Option<u16>,
    pub(crate) p2p_task_queue_capacity: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) p2p_port: u16,
    pub(crate) p2p_max_conn_count: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
    pub(crate) rpc_port: u16,
    pub(crate) secret: String,
    pub(crate) public_key_str: String,
    pub(crate) peer_table: Arc<PeerTable>,
}

impl P2PHost {
    pub(crate) async fn init(
        p2p_host_args: P2PHostArgs,
    ) -> Result<P2PHost, String> {
        let credential = {
            let id = Credential::new(
                p2p_host_args.secret,
                p2p_host_args.public_key_str,
            )?;

            tinfo!(
                "saksaha",
                "p2p",
                "Created p2p identity, public_key_str: {}",
                id.public_key_str.yellow(),
            );

            Arc::new(id)
        };

        let (p2p_task_runtime, p2p_task_queue) = {
            let p2p_task_queue = {
                let capacity = match p2p_host_args.p2p_task_queue_capacity {
                    Some(c) => c.into(),
                    None => P2P_TASK_QUEUE_CAPACITY,
                };

                let q = TaskQueue::new(capacity);
                Arc::new(q)
            };

            let runtime = P2PTaskRuntime::new(
                p2p_task_queue.clone(),
                p2p_host_args.p2p_task_interval,
            );

            (runtime, p2p_task_queue)
        };

        let (p2p_discovery, disc_port) = {
            let disc_args = DiscoveryArgs {
                addr_expire_duration: p2p_host_args.addr_expire_duration,
                disc_dial_interval: p2p_host_args.disc_dial_interval,
                disc_table_capacity: p2p_host_args.disc_table_capacity,
                disc_task_interval: p2p_host_args.disc_task_interval,
                disc_task_queue_capacity: p2p_host_args
                    .disc_task_queue_capacity,
                credential: credential.clone(),
                disc_port: p2p_host_args.disc_port,
                p2p_port: p2p_host_args.p2p_port,
                bootstrap_addrs: p2p_host_args.bootstrap_addrs,
            };

            let (disc, disc_port) = Discovery::init(disc_args).await?;

            (Arc::new(disc), disc_port)
        };

        let identity = {
            let i = Identity {
                credential: credential.clone(),
                p2p_port: p2p_host_args.p2p_port,
                disc_port,
            };

            Arc::new(i)
        };

        let p2p_server = {
            let s = Server::new(
                p2p_host_args.p2p_max_conn_count,
                p2p_host_args.p2p_socket,
                identity.clone(),
                p2p_host_args.peer_table.clone(),
            );

            s
        };

        let p2p_dial_scheduler = {
            let addrs_iter = Arc::new(p2p_discovery.new_iter());

            let p2p_dial_schd_args = P2PDialSchedulerArgs {
                p2p_dial_interval: p2p_host_args.p2p_dial_interval,
                p2p_task_queue: p2p_task_queue.clone(),
                addrs_iter,
                identity: identity.clone(),
                peer_table: p2p_host_args.peer_table.clone(),
            };

            let s = P2PDialScheduler::init(p2p_dial_schd_args);

            s
        };

        let host = P2PHost {
            p2p_discovery,
            p2p_dial_scheduler,
            p2p_task_queue,
            p2p_task_runtime,
            p2p_server,
            peer_table: p2p_host_args.peer_table.clone(),
        };

        Ok(host)
    }

    pub(crate) async fn run(&self) {
        tokio::join!(
            self.p2p_discovery.run(),
            self.p2p_task_runtime.run(),
            self.p2p_server.run(),
            self.p2p_dial_scheduler.run()
        );
    }

    pub(crate) fn get_p2p_monitor(&self) -> P2PMonitor {
        let monitor = P2PMonitor {
            peer_table: self.peer_table.clone(),
            p2p_discovery: self.p2p_discovery.clone(),
        };

        monitor
    }
}
