use super::dial_scheduler::{DialScheduler, DialSchedulerArgs};
use super::server::Server;
use super::state::DiscState;
use super::task::runtime::DiscTaskRuntime;
use super::task::DiscoveryTask;
use crate::v1::net::Connection;
use crate::{AddrVal, AddrsIterator, Table};
use colored::Colorize;
use logger::tinfo;
use p2p_identity::addr::UnknownAddr;
use p2p_identity::identity::P2PIdentity;
use std::sync::Arc;
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

            let udp_conn = Connection::new(socket);

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

        let task_runtime = self.task_runtime.clone();

        let dial_scheduler = self.dial_scheduler.clone();

        tokio::join!(
            //
            server.run(),
            task_runtime.run(),
            dial_scheduler.run(),
        );
    }

    pub fn new_iter(&self) -> AddrsIterator {
        self.disc_state.table.new_iter()
    }

    pub async fn get_status(&self) -> Vec<String> {
        let table = self.disc_state.table.clone();
        let addr_map = table.addr_map.read().await;

        let mut addr_vec = Vec::new();

        for (idx, addr) in addr_map.values().enumerate() {
            match addr.try_read() {
                Ok(addr) => {
                    println!("addr table elements [{}] - {}", idx, addr,);

                    match &addr.val {
                        AddrVal::Known(k) => {
                            let endpoint = k.disc_endpoint();
                            addr_vec.push(endpoint.clone());
                        }
                        AddrVal::Unknown(u) => {
                            let endpoint = u.disc_endpoint();
                            addr_vec.push(endpoint.clone());
                        }
                    }
                }
                Err(_err) => {
                    println!("addr table elements [{}] is locked", idx);
                }
            }
        }

        addr_vec
    }
}
