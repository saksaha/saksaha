pub mod address;
pub mod dial_scheduler;
mod frame;
pub mod iterator;
pub mod listener;
mod ops;
pub mod state;
mod table;
mod task;

use self::dial_scheduler::DialSchedulerArgs;
use self::{
    dial_scheduler::DialScheduler,
    listener::Listener,
    state::DiscState,
    table::Table,
    task::{DiscoveryTask, TaskHandler},
};
use crate::iterator::Iterator;
use colored::Colorize;
use logger::{tinfo, twarn};
use p2p_active_calls::ActiveCalls;
use p2p_identity::{peer::UnknownPeer, P2PIdentity};
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

pub const CAPACITY: usize = 64;

pub struct Discovery {
    listener: Arc<Listener>,
    disc_state: Arc<DiscState>,
    dial_scheduler: Arc<DialScheduler>,
    task_queue: Arc<TaskQueue<DiscoveryTask>>,
    task_handler: Arc<TaskHandler>,
}

pub struct DiscoveryArgs {
    pub disc_dial_interval: Option<u16>,
    pub p2p_identity: Arc<P2PIdentity>,
    pub disc_port: Option<u16>,
    pub p2p_port: u16,
    pub bootstrap_urls: Option<Vec<String>>,
    pub unknown_peers: Vec<UnknownPeer>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

impl Discovery {
    pub async fn init(disc_args: DiscoveryArgs) -> Result<Discovery, String> {
        let unknown_peers = merge_bootstrap_urls(
            disc_args.bootstrap_urls,
            disc_args.unknown_peers,
        );

        let table = {
            let t = match Table::init().await {
                Ok(t) => t,
                Err(err) => {
                    return Err(format!("Can't initialize Table, err: {}", err))
                }
            };
            Arc::new(t)
        };

        let active_calls = {
            let c = ActiveCalls::new();
            Arc::new(c)
        };

        let (udp_socket, disc_port) = {
            let (socket, port) = setup_udp_socket(disc_args.disc_port).await?;
            (Arc::new(socket), port)
        };

        let disc_state = {
            let s = DiscState {
                p2p_identity: disc_args.p2p_identity,
                table,
                active_calls,
                disc_port,
                udp_socket: udp_socket.clone(),
                p2p_port: disc_args.p2p_port,
                is_dial_routine_running: Arc::new(Mutex::new(false)),
            };
            Arc::new(s)
        };

        let task_queue = {
            let q = TaskQueue::new(10);
            Arc::new(q)
        };

        let task_handler = {
            let r = TaskHandler {
                task_queue: task_queue.clone(),
            };
            Arc::new(r)
        };

        let listener = {
            let l = Listener {
                disc_state: disc_state.clone(),
                udp_socket: udp_socket.clone(),
            };
            Arc::new(l)
        };

        let dial_schd_args = DialSchedulerArgs {
            disc_state: disc_state.clone(),
            disc_dial_interval: disc_args.disc_dial_interval,
            unknown_peers,
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
            task_handler,
        };

        Ok(disc)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.listener.start()?;

        self.task_handler.run();

        self.dial_scheduler.start()?;

        Ok(())
    }

    pub fn iter(&self) -> Arc<Iterator> {
        self.disc_state.table.iter()
    }
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

fn merge_bootstrap_urls(
    bootstrap_urls: Option<Vec<String>>,
    unknown_peers: Vec<UnknownPeer>,
) -> Vec<UnknownPeer> {
    let mut resulting_peers = Vec::from(unknown_peers);

    if let Some(urls) = bootstrap_urls {
        let mut cnt = 0;

        for url in urls {
            match UnknownPeer::parse(url.clone()) {
                Ok(p) => {
                    cnt += 1;

                    tinfo!(
                        "p2p_discovery",
                        "dial_schd",
                        "Bootstrap - [{}] {}",
                        cnt,
                        p.short_url(),
                    );

                    resulting_peers.push(p);
                }
                Err(err) => {
                    twarn!(
                        "p2p_discovery",
                        "dial_schd",
                        "Failed to parse url, url: {}, \
                            err: {:?}",
                        url.clone(),
                        err,
                    );
                }
            };
        }

        tinfo!(
            "p2p_discovery",
            "dial_schd",
            "Bootstrap - Total bootstrapped node count: {}",
            cnt,
        );
    }

    resulting_peers
}
