use super::dial_scheduler::P2PDialSchedulerArgs;
use super::state::P2PState;
use super::task::runtime::P2PTaskRuntime;
use super::task::P2PTask;
use super::{dial_scheduler::P2PDialScheduler, server::Server};
use colored::Colorize;
use logger::tinfo;
use p2p_discovery::{Discovery, DiscoveryArgs};
use p2p_identity::addr::UnknownAddr;
use p2p_identity::identity::P2PIdentity;
use p2p_peer::PeerTable;
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::TcpListener;

const P2P_TASK_QUEUE_CAPACITY: usize = 10;

pub(crate) struct P2PHost {
    p2p_state: Arc<P2PState>,
    p2p_discovery: Arc<Discovery>,
    p2p_dial_scheduler: Arc<P2PDialScheduler>,
    p2p_server: Arc<Server>,
    p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    p2p_task_runtime: Arc<P2PTaskRuntime>,
}

pub(crate) struct P2PHostArgs {
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
    pub(crate) p2p_peer_table: Arc<PeerTable>,
}

impl P2PHost {
    pub(crate) async fn init(
        p2p_host_args: P2PHostArgs,
    ) -> Result<P2PHost, String> {
        let p2p_identity = {
            let id = P2PIdentity::new(
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

            (Arc::new(runtime), p2p_task_queue)
        };

        let p2p_discovery = {
            let disc_args = DiscoveryArgs {
                disc_dial_interval: p2p_host_args.disc_dial_interval,
                disc_table_capacity: p2p_host_args.disc_table_capacity,
                disc_task_interval: p2p_host_args.disc_task_interval,
                disc_task_queue_capacity: p2p_host_args
                    .disc_task_queue_capacity,
                p2p_identity: p2p_identity.clone(),
                disc_port: p2p_host_args.disc_port,
                p2p_port: p2p_host_args.p2p_port,
                bootstrap_addrs: p2p_host_args.bootstrap_addrs,
            };

            let d = Discovery::init(disc_args).await?;

            Arc::new(d)
        };

        let p2p_state = {
            let s = P2PState {
                p2p_identity: p2p_identity.clone(),
                p2p_port: p2p_host_args.p2p_port,
                rpc_port: p2p_host_args.rpc_port,
                p2p_peer_table: p2p_host_args.p2p_peer_table.clone(),
                p2p_discovery: p2p_discovery.clone(),
            };

            Arc::new(s)
        };

        let p2p_server = {
            let s = Server::new(
                p2p_state.clone(),
                p2p_host_args.p2p_max_conn_count,
                p2p_host_args.p2p_socket,
            );

            Arc::new(s)
        };

        let p2p_dial_scheduler = {
            let addrs_iter = Arc::new(p2p_discovery.new_iter());

            let p2p_dial_schd_args = P2PDialSchedulerArgs {
                p2p_state: p2p_state.clone(),
                p2p_dial_interval: p2p_host_args.p2p_dial_interval,
                p2p_task_queue: p2p_task_queue.clone(),
                addrs_iter,
            };

            let s = P2PDialScheduler::init(p2p_dial_schd_args);

            Arc::new(s)
        };

        let host = P2PHost {
            p2p_discovery,
            p2p_dial_scheduler,
            p2p_task_queue,
            p2p_task_runtime,
            p2p_server,
            p2p_state,
        };

        Ok(host)
    }

    pub(crate) async fn run(&self) {
        let disc = self.p2p_discovery.clone();

        // let p2p_task_runtime = self.p2p_task_runtime.clone();

        // let p2p_server = self.p2p_server.clone();

        // let p2p_dial_scheduler = self.p2p_dial_scheduler.clone();

        tokio::join!(
            disc.run(),
            // p2p_task_runtime.run(),
            // p2p_server.run(),
            // p2p_dial_scheduler.run()
        );
    }

    pub(crate) fn get_p2p_state(&self) -> Arc<P2PState> {
        return self.p2p_state.clone();
    }
}
