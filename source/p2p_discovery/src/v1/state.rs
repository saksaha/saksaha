use crate::task::DiscoveryTask;
use p2p_active_calls::ActiveCalls;
use p2p_identity::P2PIdentity;
use std::sync::Arc;
use task::task_queue::TaskQueue;
use tokio::net::UdpSocket;

use super::table::Table;

pub(crate) struct DiscState {
    pub p2p_identity: Arc<P2PIdentity>,
    pub udp_socket: Arc<UdpSocket>,
    pub disc_port: u16,
    pub p2p_port: u16,
    pub table: Arc<Table>,
    pub active_calls: Arc<ActiveCalls>,
    // pub task_queue: Arc<TaskQueue<Task>>,
}

// impl DiscState {
//     pub fn new(
//         identity: Arc<P2PIdentity>,
//         table: Arc<Table>,
//         active_calls: Arc<ActiveCalls>,
//         // udp_socket: Arc<UdpSocket>,
//         my_disc_port: u16,
//         my_p2p_port: u16,
//         // task_queue: Arc<TaskQueue<Task>>,
//     ) -> DiscState {
//         DiscState {
//             identity,
//             udp_socket,
//             my_disc_port,
//             my_p2p_port,
//             table,
//             // task_queue,
//             _active_calls: active_calls,
//         }
//     }
// }
