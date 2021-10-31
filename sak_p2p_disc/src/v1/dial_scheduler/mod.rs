mod handler;

use crate::v1::{
    dial_scheduler::handler::HandleError, table::Table, task_queue::TaskQueue,
};
use handler::Handler;
use log::{debug, error, info, warn};
use sak_p2p_identity::Identity;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub struct DialScheduler {}

impl DialScheduler {
    pub fn new() -> DialScheduler {
        DialScheduler {}
    }

    pub fn start(
        &self,
        id: Arc<Box<dyn Identity + Send + Sync>>,
        my_disc_port: u16,
        my_peer_op_port: u16,
        table: Arc<Table>,
        task_queue: Arc<TaskQueue>,
    ) -> Result<(), String> {
        info!("Started - Discovery dial scheduler");

        Ok(())
    }
}

// pub struct Routine {
//     id: Arc<dyn Identity>,
//     my_p2p_listener_port: u16,
//     my_disc_port: u16,
//     is_running: Arc<Mutex<bool>>,
//     table: Arc<Table>,
// }

// impl Routine {
//     pub fn new(
//         id: Arc<dyn Identity>,
//         my_disc_port: u16,
//         my_p2p_listener_port: u16,
//         table: Arc<Table>,
//     ) -> Routine {
//         let is_running = Arc::new(Mutex::new(false));

//         Routine {
//             id,
//             my_disc_port,
//             my_p2p_listener_port,
//             is_running,
//             table,
//         }
//     }

//     pub fn run(&self) {
//         let is_running = self.is_running.clone();
//         let my_disc_port = self.my_disc_port;
//         let my_p2p_listener_port = self.my_p2p_listener_port;
//         let table = self.table.clone();

//         tokio::spawn(async move {
//             let mut is_running_lock = is_running.lock().await;
//             *is_running_lock = true;
//             std::mem::drop(is_running_lock);

//             loop {
//                 let start = SystemTime::now();
//                 let handler = Handler::new();
//                 let handler_run = handler.run(
//                     my_disc_port,
//                     my_p2p_listener_port,
//                     table.clone(),
//                 );

//                 match handler_run.await {
//                     Ok(_) => (),
//                     Err(err) => match err {
//                         HandleError::IllegalEndpoint(err) => {
//                             warn!(
//                                 "Peer may have an illegal endpoint, err: {}",
//                                 err
//                             );
//                         }
//                         HandleError::NoAvailableNode => {
//                             warn!("No available node to discover");
//                             break;
//                         }
//                         HandleError::IllegalPeerFound(idx) => {
//                             warn!("Illegal peer has been found, idx: {}", idx,);
//                         }
//                         HandleError::ConnectionFail(err) => {
//                             warn!("Disc dial connection fail, err: {}", err);
//                         }
//                         HandleError::LocalAddrIdentical => (),
//                         HandleError::WhoAreYouInitiateFail(err) => {
//                             warn!(
//                                 "Disc dial who are you \
//                                     initiate failed, err: {}",
//                                 err
//                             );
//                         }
//                         HandleError::WhoAreYouAckReceiveFail(err) => {
//                             warn!(
//                                 "Disc dial who are you \
//                                     ack receive failed, err: {}",
//                                 err
//                             );
//                         }
//                         HandleError::PeerUpdateFail(err) => {
//                             warn!("Disc dial peer update fail, err: {}", err);
//                         }
//                     },
//                 }

//                 tokio::time::sleep(Duration::from_millis(100)).await;

//                 match start.elapsed() {
//                     Ok(_) => (),
//                     Err(err) => {
//                         warn!("Error sleeping the duration, err: {}", err);
//                     }
//                 };
//             }

//             let mut is_running_lock = is_running.lock().await;
//             *is_running_lock = false;
//         });
//     }

// pub async fn wakeup(&self) {
//     let is_running = self.is_running.lock().await;

//     if *is_running == false {
//         warn!("Disc dial routine is not running, waking up");

//         self.run();
//     }
// }
// }
