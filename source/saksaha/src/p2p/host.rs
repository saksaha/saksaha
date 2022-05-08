use super::dial_scheduler::P2PDialSchedulerArgs;
use super::state::HostState;
use super::task::runtime::P2PTaskRuntime;
use super::task::P2PTask;
use super::{dial_scheduler::P2PDialScheduler, server::Server};
use colored::Colorize;
use logger::tinfo;
use p2p_active_calls::ActiveCalls;
use p2p_discovery::{Discovery, DiscoveryArgs};
use p2p_identity::addr::UnknownAddr;
use p2p_identity::identity::P2PIdentity;
use p2p_peer::PeerTable;
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::TcpListener;

const P2P_TASK_QUEUE_CAPACITY: usize = 10;

pub(crate) struct Host {
    host_state: Arc<HostState>,
    discovery: Arc<Discovery>,
    p2p_active_calls: Arc<ActiveCalls>,
    p2p_dial_scheduler: Arc<P2PDialScheduler>,
    p2p_server: Arc<Server>,
    p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    p2p_task_runtime: Arc<P2PTaskRuntime>,
}

pub(crate) struct HostArgs {
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

impl Host {
    pub async fn init(host_args: HostArgs) -> Result<Host, String> {
        let p2p_identity = {
            let id =
                P2PIdentity::new(host_args.secret, host_args.public_key_str)?;

            tinfo!(
                "saksaha",
                "p2p",
                "Created p2p identity, public_key_str: {}",
                id.public_key_str.yellow(),
            );

            Arc::new(id)
        };

        let p2p_task_queue = {
            let capacity = match host_args.p2p_task_queue_capacity {
                Some(c) => c.into(),
                None => P2P_TASK_QUEUE_CAPACITY,
            };

            let q = TaskQueue::new(capacity);
            Arc::new(q)
        };

        let p2p_task_runtime = {
            let h = P2PTaskRuntime::new(
                p2p_task_queue.clone(),
                host_args.p2p_task_interval,
            );
            Arc::new(h)
        };

        let p2p_active_calls = {
            let a = ActiveCalls::init().await;

            Arc::new(a)
        };

        let host_state = {
            let s = HostState {
                p2p_active_calls: p2p_active_calls.clone(),
                p2p_identity: p2p_identity.clone(),
                p2p_port: host_args.p2p_port,
                rpc_port: host_args.rpc_port,
                peer_table: host_args.peer_table.clone(),
            };
            Arc::new(s)
        };

        let p2p_server = {
            let s = Server::new(
                host_state.clone(),
                host_args.p2p_max_conn_count,
                host_args.p2p_socket,
            );
            Arc::new(s)
        };

        let disc_args = DiscoveryArgs {
            disc_dial_interval: host_args.disc_dial_interval,
            disc_table_capacity: host_args.disc_table_capacity,
            disc_task_interval: host_args.disc_task_interval,
            disc_task_queue_capacity: host_args.disc_task_queue_capacity,
            p2p_identity: p2p_identity.clone(),
            disc_port: host_args.disc_port,
            p2p_port: host_args.p2p_port,
            bootstrap_addrs: host_args.bootstrap_addrs,
        };

        let discovery = {
            let d = Discovery::init(disc_args).await?;
            Arc::new(d)
        };

        let p2p_dial_scheduler = {
            let addrs_iter = Arc::new(discovery.iter());

            let p2p_dial_schd_args = P2PDialSchedulerArgs {
                host_state: host_state.clone(),
                p2p_dial_interval: host_args.p2p_dial_interval,
                p2p_task_queue: p2p_task_queue.clone(),
                addrs_iter,
            };

            let s = P2PDialScheduler::init(p2p_dial_schd_args);

            Arc::new(s)
        };

        let host = Host {
            discovery,
            p2p_active_calls,
            p2p_dial_scheduler,
            p2p_task_queue,
            p2p_task_runtime,
            p2p_server,
            host_state,
        };

        Ok(host)
    }

    pub async fn run(&self) {
        let disc = self.discovery.clone();
        tokio::spawn(async move {
            disc.run().await;
        });

        let p2p_task_runtime = self.p2p_task_runtime.clone();
        tokio::spawn(async move {
            p2p_task_runtime.run().await;
        });

        let p2p_server = self.p2p_server.clone();
        tokio::spawn(async move {
            p2p_server.run().await;
        });

        let p2p_dial_scheduler = self.p2p_dial_scheduler.clone();
        tokio::spawn(async move {
            p2p_dial_scheduler.run().await;
        });
    }
}
