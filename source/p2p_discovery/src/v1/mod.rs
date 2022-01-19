pub mod address;
pub mod dial_scheduler;
pub mod iterator;
pub mod listener;
mod operations;
pub mod state;
mod table;
pub mod task;

use self::{
    dial_scheduler::DialScheduler, listener::Listener,
    state::DiscState,
    table::Table,
};
use crate::{iterator::Iterator, task::DiscTaskRunner};
use ::task::task_queue::TaskQueue;
use colored::*;
use logger::tinfo;
use p2p_active_calls::ActiveCalls;
use p2p_identity::Identity;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub const CAPACITY: usize = 64;

pub struct Discovery {
    listener: Arc<Listener>,
    disc_state: Arc<DiscState>,
    dial_scheduler: Arc<DialScheduler>,
}

impl Discovery {
    pub async fn init(
        id: Arc<Identity>,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Discovery, String> {
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

        let task_queue = {
            let q = TaskQueue::new(
                "p2p_discovery".to_string(),
                Box::new(DiscTaskRunner {}),
            );
            Arc::new(q)
        };

        let (udp_socket, my_disc_port) = {
            let (socket, port) = setup_udp_socket(my_disc_port).await?;
            (Arc::new(socket), port)
        };

        let disc_state = {
            let s = DiscState::new(
                id,
                table,
                active_calls,
                udp_socket.clone(),
                my_disc_port,
                my_p2p_port,
                task_queue.clone(),
            );
            Arc::new(s)
        };

        // let whoareyou_op = {
        //     let w = WhoareyouOp::new(disc_state.clone());
        //     Arc::new(w)
        // };

        let listener = {
            let l = Listener::new(
                disc_state.clone(),
                udp_socket.clone(),
                // whoareyou_op.clone(),
            );
            Arc::new(l)
        };

        let dial_scheduler = {
            let s = DialScheduler::init(
                disc_state.clone(),
                // whoareyou_op.clone(),
                bootstrap_urls,
                default_bootstrap_urls,
            )
            .await;
            Arc::new(s)
        };

        let disc = Discovery {
            disc_state,
            listener,
            dial_scheduler,
        };

        Ok(disc)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.listener.start()?;
        self.dial_scheduler.start()?;
        self.disc_state.task_queue.run_loop();

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
                "Bound udp socket for discovery, addr: {}",
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
