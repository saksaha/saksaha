mod active_calls;
pub mod address;
pub mod dial_scheduler;
pub mod iterator;
pub mod listener;
mod ops;
mod table;
pub mod task;

use self::{
    active_calls::ActiveCalls, dial_scheduler::DialScheduler,
    listener::Listener, ops::whoareyou::WhoareyouOp, table::Table,
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
    task_queue: Arc<TaskQueue<Task>>,
    listener: Arc<Listener>,
    state: Arc<DiscState>,
    dial_scheduler: Arc<DialScheduler>,
}

impl Disc {
    pub async fn init(
        id: Arc<Box<dyn Identity + Send + Sync>>,
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

        let active_calls = Arc::new(ActiveCalls::new());
        let task_queue = Arc::new(TaskQueue::new(Box::new(TaskRunner {})));

        let (udp_socket, port) = {
            let (socket, port) = setup_udp_socket(my_disc_port).await?;
            (Arc::new(socket), port)
        };

        let state = {
            let s = DiscState::new(
                id,
                table,
                active_calls,
                port,
                my_p2p_port,
            );
            Arc::new(s)
        };

        let whoareyou_op = {
            let w = WhoareyouOp::new(udp_socket.clone(), state.clone());
            Arc::new(w)
        };

        let listener = {
            let l = Listener::new(
                state.clone(),
                udp_socket.clone(),
                whoareyou_op.clone(),
                task_queue.clone(),
            );
            Arc::new(l)
        };

        let dial_scheduler = {
            let d = DialScheduler::new(
                state.clone(),
                task_queue.clone(),
                whoareyou_op.clone(),
            );
            Arc::new(d)
        };

        dial_scheduler
            .enqueue_initial_tasks(bootstrap_urls, default_bootstrap_urls)
            .await;

        let disc = Disc {
            task_queue,
            state,
            listener,
            dial_scheduler,
        };

        Ok(disc)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.listener.start()?;
        self.dial_scheduler.start()?;
        self.task_queue.run_loop();

        Ok(())
    }

    pub fn iter(&self) -> Arc<Iterator> {
        self.state.table.iter()
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

pub(crate) struct DiscState {
    id: Arc<Box<dyn Identity + Send + Sync>>,
    my_disc_port: u16,
    my_p2p_port: u16,
    table: Arc<Table>,
    _active_calls: Arc<ActiveCalls>,
}

impl DiscState {
    pub fn new(
        id: Arc<Box<dyn Identity + Send + Sync>>,
        table: Arc<Table>,
        active_calls: Arc<ActiveCalls>,
        my_disc_port: u16,
        my_p2p_port: u16,
    ) -> DiscState {
        DiscState {
            id,
            my_disc_port,
            my_p2p_port,
            table,
            _active_calls: active_calls,
        }
    }
}
