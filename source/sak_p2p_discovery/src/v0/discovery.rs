use super::dial_scheduler::{DialScheduler, DialSchedulerArgs};
use super::server::{Server, ServerArgs};
use super::task::runtime::DiscTaskRuntime;
use crate::{AddrTable, Connection, DiscRuntime};
use colored::Colorize;
use log::info;
use sak_p2p_addr::UnknownAddr;
use sak_p2p_id::Identity;
use sak_task_queue::TaskQueue;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;

const DISC_TASK_QUEUE_CAPACITY: usize = 10;
const ADDR_EXPIRE_DURATION: u64 = 3600;
const ADDR_MONITOR_INTERVAL: u64 = 3000;

pub struct Discovery {
    disc_runtime: DiscRuntime,
    server: Server,
    dial_scheduler: DialScheduler,
    task_runtime: DiscTaskRuntime,
    pub addr_table: Arc<AddrTable>,
}

pub struct DiscoveryArgs {
    pub addr_expire_duration: Option<u64>,
    pub addr_monitor_interval: Option<u64>,
    pub disc_dial_interval: Option<u16>,
    pub disc_table_capacity: Option<u16>,
    pub disc_task_interval: Option<u16>,
    pub disc_task_queue_capacity: Option<u16>,
    pub p2p_port: u16,
    pub bootstrap_addrs: Vec<UnknownAddr>,
    pub udp_socket: UdpSocket,
    pub identity: Arc<Identity>,
}

impl Discovery {
    pub async fn init(
        disc_args: DiscoveryArgs,
    ) -> Result<(Discovery, u16), String> {
        let (udp_conn, disc_port) = {
            let socket_addr = match disc_args.udp_socket.local_addr() {
                Ok(a) => a,
                Err(err) => {
                    return Err(format!(
                        "Cannot retrieve addr of udp socket, err: {}",
                        err
                    ));
                }
            };

            let udp_conn = Connection::new(disc_args.udp_socket);

            info!(
                "Bound udp socket for P2P discovery, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (Arc::new(udp_conn), socket_addr.port())
        };

        let addr_expire_duration = match disc_args.addr_expire_duration {
            Some(d) => d,
            None => ADDR_EXPIRE_DURATION,
        };

        let addr_monitor_interval = match disc_args.addr_monitor_interval {
            Some(d) => Duration::from_millis(d),
            None => Duration::from_millis(ADDR_MONITOR_INTERVAL),
        };

        let addr_table = {
            let t = match AddrTable::init(disc_args.disc_table_capacity).await {
                Ok(t) => t,
                Err(err) => {
                    return Err(
                        format!("Can't initialize Table, err: {}", err).into()
                    )
                }
            };

            Arc::new(t)
        };

        let disc_task_queue = {
            let capacity = match disc_args.disc_task_queue_capacity {
                Some(c) => c.into(),
                None => DISC_TASK_QUEUE_CAPACITY,
            };

            let q = TaskQueue::new(capacity);
            Arc::new(q)
        };

        let dial_schd_args = DialSchedulerArgs {
            disc_dial_interval: disc_args.disc_dial_interval,
            bootstrap_addrs: disc_args.bootstrap_addrs,
            disc_task_queue: disc_task_queue.clone(),
        };

        let dial_scheduler = {
            let s = DialScheduler::init(dial_schd_args);

            s
        };

        let server = {
            let server_args = ServerArgs {
                udp_conn: udp_conn.clone(),
                identity: disc_args.identity.clone(),
                addr_table: addr_table.clone(),
                addr_expire_duration,
            };

            let s = Server::new(server_args);

            s
        };

        let disc_runtime = {
            let r = DiscRuntime {
                addr_monitor_interval,
                addr_table: addr_table.clone(),
            };

            r
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(
                disc_task_queue.clone(),
                disc_args.disc_task_interval,
                disc_args.identity.clone(),
                addr_table.clone(),
                udp_conn,
            );

            h
        };

        let disc = Discovery {
            server,
            task_runtime,
            dial_scheduler,
            addr_table,
            disc_runtime,
        };

        Ok((disc, disc_port))
    }

    pub async fn enqueue_who_are_you(&self, unknown_addr: &UnknownAddr) {
        self.dial_scheduler.enqueue_who_are_you(unknown_addr).await;
    }

    pub async fn run(&self) {
        tokio::join!(
            //
            self.server.run(),
            self.task_runtime.run(),
            self.dial_scheduler.run(),
            self.disc_runtime.run(),
        );
    }
}
