use crate::task::Task;
use p2p_active_calls::ActiveCalls;
use p2p_identity::Identity;
use task::task_queue::TaskQueue;
use std::sync::Arc;
use tokio::net::UdpSocket;

use super::table::Table;

pub(crate) struct DiscState {
    pub identity: Arc<Identity>,
    pub udp_socket: Arc<UdpSocket>,
    pub my_disc_port: u16,
    pub my_p2p_port: u16,
    pub table: Arc<Table>,
    // pub task_queue: Arc<TaskQueue<Task>>,
    pub _active_calls: Arc<ActiveCalls>,
}

impl DiscState {
    pub fn new(
        identity: Arc<Identity>,
        table: Arc<Table>,
        active_calls: Arc<ActiveCalls>,
        udp_socket: Arc<UdpSocket>,
        my_disc_port: u16,
        my_p2p_port: u16,
        // task_queue: Arc<TaskQueue<Task>>,
    ) -> DiscState {
        DiscState {
            identity,
            udp_socket,
            my_disc_port,
            my_p2p_port,
            table,
            // task_queue,
            _active_calls: active_calls,
        }
    }
}
