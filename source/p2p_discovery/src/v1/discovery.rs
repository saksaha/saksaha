use super::addr_monitor_routine::AddrMonitorRoutine;
use super::dial_scheduler::{DialScheduler, DialSchedulerArgs};
use super::server::{Server, ServerArgs};
use super::task::runtime::DiscTaskRuntime;
use crate::v1::net::Connection;
use crate::AddrTable;
use colored::Colorize;
use logger::tinfo;
use p2p_addr::UnknownAddr;
use p2p_identity::{Credential, Identity};
use std::sync::Arc;
use std::time::Duration;
use task_queue::TaskQueue;

const DISC_TASK_QUEUE_CAPACITY: usize = 10;
const ADDR_EXPIRE_DURATION: u64 = 3600;
const ADDR_MONITOR_INTERVAL: u64 = 1000;

pub struct Discovery {
    addr_monitor_routine: AddrMonitorRoutine,
    server: Server,
    dial_scheduler: DialScheduler,
    task_runtime: DiscTaskRuntime,
    pub addr_table: Arc<AddrTable>,
}

#[derive(Clone)]
pub struct DiscoveryArgs {
    pub addr_expire_duration: Option<u64>,
    pub addr_monitor_interval: Option<u64>,
    pub disc_dial_interval: Option<u16>,
    pub disc_table_capacity: Option<u16>,
    pub disc_task_interval: Option<u16>,
    pub disc_task_queue_capacity: Option<u16>,
    pub credential: Arc<Credential>,
    pub disc_port: Option<u16>,
    pub p2p_port: u16,
    pub bootstrap_addrs: Vec<UnknownAddr>,
}

impl Discovery {
    pub async fn init(
        disc_args: DiscoveryArgs,
    ) -> Result<(Discovery, u16), String> {
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

        let addr_expire_duration = match disc_args.addr_expire_duration {
            Some(d) => d,
            None => ADDR_EXPIRE_DURATION,
        };

        let addr_monitor_interval = match disc_args.addr_monitor_interval {
            Some(d) => Duration::from_millis(d),
            None => Duration::from_millis(ADDR_MONITOR_INTERVAL),
        };

        let identity = {
            let i = Identity {
                credential: disc_args.credential,
                p2p_port: disc_args.p2p_port,
                disc_port,
            };

            Arc::new(i)
        };

        let addr_table = {
            let t = match AddrTable::init(disc_args.disc_table_capacity).await {
                Ok(t) => t,
                Err(err) => {
                    return Err(format!("Can't initialize Table, err: {}", err))
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
                identity: identity.clone(),
                addr_table: addr_table.clone(),
                addr_expire_duration,
            };

            let s = Server::new(server_args);

            s
        };

        let addr_monitor_routine = {
            let r = AddrMonitorRoutine {
                addr_monitor_interval,
                addr_table: addr_table.clone(),
            };

            r
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(
                disc_task_queue.clone(),
                disc_args.disc_task_interval,
                identity.clone(),
                addr_table.clone(),
                udp_conn.clone(),
            );

            h
        };

        let disc = Discovery {
            server,
            task_runtime,
            dial_scheduler,
            addr_table,
            addr_monitor_routine,
        };

        Ok((disc, disc_port))
    }

    pub async fn run(&self) {
        tokio::join!(
            //
            self.server.run(),
            self.task_runtime.run(),
            self.dial_scheduler.run(),
            self.addr_monitor_routine.run(),
        );
    }

    pub async fn get_status(&self) -> Vec<String> {
        let addrs = self.addr_table.get_all_addrs_str().await;
        addrs
    }
}
