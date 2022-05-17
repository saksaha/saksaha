pub(crate) mod dial_scheduler;
pub(crate) mod msg;
mod net;
pub(crate) mod ops;
pub(crate) mod server;
pub(crate) mod state;
pub(crate) mod table;
mod task;
#[cfg(test)]
mod test;

use self::dial_scheduler::DialSchedulerArgs;
use self::net::connection::UdpConn;
use self::task::DiscoveryTask;
use self::{
    dial_scheduler::DialScheduler, server::Server, state::DiscState,
    task::runtime::DiscTaskRuntime,
};
use colored::Colorize;
use logger::tinfo;
use p2p_identity::addr::UnknownAddr;
use p2p_identity::identity::P2PIdentity;
use std::sync::Arc;
pub use table::*;
use task_queue::TaskQueue;

const DISC_TASK_QUEUE_CAPACITY: usize = 10;

pub struct Discovery {
    server: Arc<Server>,
    disc_state: Arc<DiscState>,
    dial_scheduler: Arc<DialScheduler>,
    disc_task_queue: Arc<TaskQueue<DiscoveryTask>>,
    task_runtime: Arc<DiscTaskRuntime>,
}

pub struct DiscoveryArgs {
    pub disc_dial_interval: Option<u16>,
    pub disc_table_capacity: Option<u16>,
    pub disc_task_interval: Option<u16>,
    pub disc_task_queue_capacity: Option<u16>,
    pub p2p_identity: Arc<P2PIdentity>,
    pub disc_port: Option<u16>,
    pub p2p_port: u16,
    pub bootstrap_addrs: Vec<UnknownAddr>,
}

impl Discovery {
    pub async fn init(disc_args: DiscoveryArgs) -> Result<Discovery, String> {
        let table = {
            let t = match Table::init(disc_args.disc_table_capacity).await {
                Ok(t) => t,
                Err(err) => {
                    return Err(format!("Can't initialize Table, err: {}", err))
                }
            };

            Arc::new(t)
        };

        let (udp_conn, disc_port) = {
            let (socket, socket_addr) =
                utils_net::setup_udp_socket(disc_args.disc_port).await?;
            let udp_conn = UdpConn { socket };

            tinfo!(
                "p2p_discovery",
                "",
                "Bound udp socket for P2P discovery, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (Arc::new(udp_conn), socket_addr.port())
        };

        let disc_state = {
            let s = DiscState {
                p2p_identity: disc_args.p2p_identity,
                table,
                disc_port,
                udp_conn,
                p2p_port: disc_args.p2p_port,
            };
            Arc::new(s)
        };

        let disc_task_queue = {
            let capacity = match disc_args.disc_task_queue_capacity {
                Some(c) => c.into(),
                None => DISC_TASK_QUEUE_CAPACITY,
            };

            let q = TaskQueue::new(capacity);
            Arc::new(q)
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(
                disc_task_queue.clone(),
                disc_args.disc_task_interval,
            );
            Arc::new(h)
        };

        let server = {
            let s = Server::new(disc_state.clone());
            Arc::new(s)
        };

        let dial_schd_args = DialSchedulerArgs {
            disc_state: disc_state.clone(),
            disc_dial_interval: disc_args.disc_dial_interval,
            bootstrap_addrs: disc_args.bootstrap_addrs,
            disc_task_queue: disc_task_queue.clone(),
        };

        let dial_scheduler = {
            let s = DialScheduler::init(dial_schd_args);
            Arc::new(s)
        };

        let disc = Discovery {
            disc_state,
            server,
            dial_scheduler,
            disc_task_queue,
            task_runtime,
        };

        Ok(disc)
    }

    pub async fn run(&self) {
        let server = self.server.clone();
        let server_thread = tokio::spawn(async move {
            server.run().await;
        });

        let task_runtime = self.task_runtime.clone();
        let task_runtime_thread = tokio::spawn(async move {
            task_runtime.run().await;
        });

        let dial_scheduler = self.dial_scheduler.clone();
        let dial_scheduler_thread = tokio::spawn(async move {
            dial_scheduler.run().await;
        });

        tokio::join!(server_thread, task_runtime_thread, dial_scheduler_thread);
    }

    pub fn new_iter(&self) -> AddrsIterator {
        self.disc_state.table.new_iter()
    }
}
