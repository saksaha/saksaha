mod active_calls;
pub mod address;
pub mod dial_scheduler;
pub mod iterator;
pub mod listener;
mod ops;
pub mod state;
mod table;
pub mod task;

use self::{
    active_calls::ActiveCalls, dial_scheduler::DialScheduler,
    listener::Listener, ops::whoareyou::WhoareyouOp, state::DiscState,
    table::Table,
};
use crate::{
    iterator::Iterator,
    task::TaskRunner,
    v1::{address::Address, task::Task},
};
use log::{info, warn};
use saksaha_p2p_identity::Identity;
use saksaha_task::task_queue::TaskQueue;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub const CAPACITY: usize = 32;

pub struct Disc {
    listener: Arc<Listener>,
    disc_state: Arc<DiscState>,
    dial_scheduler: Arc<DialScheduler>,
}

impl Disc {
    pub async fn init(
        id: Arc<Identity>,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: String,
    ) -> Result<Disc, String> {
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
            let q = TaskQueue::new("Disc".to_string(), Box::new(TaskRunner {}));
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
                my_disc_port,
                my_p2p_port,
                task_queue.clone(),
            );
            Arc::new(s)
        };

        let whoareyou_op = {
            let w = WhoareyouOp::new(udp_socket.clone(), disc_state.clone());
            Arc::new(w)
        };

        let listener = {
            let l = Listener::new(
                disc_state.clone(),
                udp_socket.clone(),
                whoareyou_op.clone(),
            );
            Arc::new(l)
        };

        let dial_scheduler = {
            let s = DialScheduler::init(
                disc_state.clone(),
                whoareyou_op.clone(),
                bootstrap_urls,
                default_bootstrap_urls,
            ).await;
            Arc::new(s)
        };

        let disc = Disc {
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

            info!(
                "Started - Discovery udp socket opened, local_addr: {}",
                local_addr
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
