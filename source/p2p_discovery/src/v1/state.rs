use super::task::DiscoveryTask;
use p2p_active_calls::ActiveCalls;
use p2p_identity::P2PIdentity;
use std::sync::Arc;
use task_queue::TaskQueue;
use tokio::net::UdpSocket;

use super::table::Table;

pub(crate) struct DiscState {
    pub(crate) p2p_identity: Arc<P2PIdentity>,
    pub(crate) udp_socket: Arc<UdpSocket>,
    pub(crate) disc_port: u16,
    pub(crate) p2p_port: u16,
    pub(crate) table: Arc<Table>,
    pub(crate) active_calls: Arc<ActiveCalls>,
    pub(crate) is_dial_routine_running: Arc<Mutex<bool>>,
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
