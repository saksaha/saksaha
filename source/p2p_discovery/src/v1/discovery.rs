use super::dial_scheduler::{DialScheduler, DialSchedulerArgs};
use super::server::{Server, ServerArgs};
use super::task::runtime::DiscTaskRuntime;
use crate::v1::net::Connection;
use crate::{AddrsIterator, Table};
use colored::Colorize;
use logger::tinfo;
use p2p_addr::UnknownAddr;
use p2p_identity::{Credential, Identity};
use std::sync::Arc;
use task_queue::TaskQueue;

const DISC_TASK_QUEUE_CAPACITY: usize = 10;
const ADDR_EXPIRE_DURATION: i64 = 3600;

pub struct Discovery {
    server: Server,
    dial_scheduler: DialScheduler,
    task_runtime: DiscTaskRuntime,
    table: Arc<Table>,
}

pub struct DiscoveryArgs {
    pub addr_expire_duration: Option<i64>,
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

        let identity = {
            let i = Identity {
                credential: disc_args.credential,
                p2p_port: disc_args.p2p_port,
                disc_port,
            };

            Arc::new(i)
        };

        let table = {
            let t = match Table::init(disc_args.disc_table_capacity).await {
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
                table: table.clone(),
                addr_expire_duration,
            };

            let s = Server::new(server_args);

            s
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(
                disc_task_queue.clone(),
                disc_args.disc_task_interval,
                identity.clone(),
                table.clone(),
                udp_conn.clone(),
            );

            h
        };

        let disc = Discovery {
            server,
            task_runtime,
            dial_scheduler,
            table,
        };

        Ok((disc, disc_port))
    }

    pub async fn run(&self) {
        tokio::join!(
            //
            self.server.run(),
            self.task_runtime.run(),
            self.dial_scheduler.run(),
        );
    }

    pub fn new_iter(&self) -> AddrsIterator {
        self.table.new_iter()
    }

    pub async fn get_status(&self) -> Vec<String> {
        let table = self.table.clone();
        let addr_map = table.addr_map.read().await;

        let mut addr_vec = Vec::new();

        for (idx, addr) in addr_map.values().enumerate() {
            match addr.try_read() {
                Ok(addr) => {
                    println!("addr table elements [{}] - {}", idx, addr,);

                    addr_vec.push(addr.known_addr.disc_endpoint())

                    // match &addr.val {
                    //     AddrVal::Known(k) => {
                    //         addr_vec.push(k.disc_endpoint());
                    //     }
                    //     AddrVal::Unknown(u) => {
                    //         addr_vec.push(u.disc_endpoint());
                    //     }
                    // }
                }
                Err(_err) => {
                    println!("addr table elements [{}] is locked", idx);
                }
            }
        }

        addr_vec
    }
}
