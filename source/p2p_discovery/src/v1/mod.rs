pub(crate) mod dial_scheduler;
mod instr;
pub(crate) mod iterator;
pub(crate) mod listener;
pub(crate) mod msg;
mod net;
pub(crate) mod state;
pub(crate) mod table;
mod task;

use self::dial_scheduler::DialSchedulerArgs;
use self::net::connection::UdpConn;
use self::task::DiscoveryTaskInstance;
use self::{
    dial_scheduler::DialScheduler, listener::Listener, state::DiscState,
    task::runtime::DiscTaskRuntime, task::DiscoveryTask,
};
use crate::iterator::Iterator;
use colored::Colorize;
use logger::{tinfo, twarn};
use p2p_active_calls::ActiveCalls;
use p2p_identity::addr::Addr;
use p2p_identity::{identity::P2PIdentity, peer::UnknownPeer};
use std::sync::Arc;
use table::Table;
use task_queue::TaskQueue;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

pub const CAPACITY: usize = 64;

pub struct Discovery {
    listener: Arc<Listener>,
    disc_state: Arc<DiscState>,
    dial_scheduler: Arc<DialScheduler>,
    task_queue: Arc<TaskQueue<DiscoveryTaskInstance>>,
    task_runtime: Arc<DiscTaskRuntime>,
}

pub struct DiscoveryArgs {
    pub disc_dial_interval: Option<u16>,
    pub disc_table_capacity: Option<u16>,
    pub disc_task_interval: Option<u16>,
    pub p2p_identity: Arc<P2PIdentity>,
    pub disc_port: Option<u16>,
    pub p2p_port: u16,
    pub bootstrap_addrs: Vec<Addr>,
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
            let (socket, port) = setup_udp_socket(disc_args.disc_port).await?;
            let udp_conn = UdpConn { socket };
            (Arc::new(udp_conn), port)
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

        let task_queue = {
            let q = TaskQueue::new(10);
            Arc::new(q)
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(
                task_queue.clone(),
                disc_args.disc_task_interval,
            );
            Arc::new(h)
        };

        let listener = {
            let l = Listener::new(disc_state.clone());
            Arc::new(l)
        };

        let dial_schd_args = DialSchedulerArgs {
            disc_state: disc_state.clone(),
            disc_dial_interval: disc_args.disc_dial_interval,
            bootstrap_addrs: disc_args.bootstrap_addrs,
            task_queue: task_queue.clone(),
        };

        let dial_scheduler = {
            let s = DialScheduler::init(dial_schd_args).await;
            Arc::new(s)
        };

        let disc = Discovery {
            disc_state,
            listener,
            dial_scheduler,
            task_queue,
            task_runtime,
        };

        Ok(disc)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.listener.start()?;

        self.task_runtime.run();

        self.dial_scheduler.start().await?;

        Ok(())
    }

    // pub fn iter(&self) -> Arc<Iterator> {
    //     self.disc_state.table.iter()
    // }
}

pub async fn setup_udp_socket(
    my_disc_port: Option<u16>,
) -> Result<(UdpSocket, u16), String> {
    let my_disc_port = match my_disc_port {
        Some(p) => p,
        None => 0,
    };

    let local_addr = format!("127.0.0.1:{}", my_disc_port);

    let (udp_socket, port) = match UdpSocket::bind(local_addr).await {
        Ok(s) => {
            let local_addr = match s.local_addr() {
                Ok(a) => a,
                Err(err) => {
                    return Err(format!(
                        "Couldn't get local address of udp socket, err: {}",
                        err
                    ))
                }
            };

            tinfo!(
                "p2p_discovery",
                "",
                "Bound udp socket for P2P discovery, addr: {}",
                local_addr.to_string().yellow(),
            );

            (s, local_addr.port())
        }
        Err(err) => {
            return Err(format!(
                "Couldn't open UdpSocket, err: {}",
                err.to_string()
            ));
        }
    };

    Ok((udp_socket, port))
}
