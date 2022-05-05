// // use crate::p2p::task::HSInitTaskParams; // use super::{state::HostState, task::Task}; use log::{debug, error, info, warn};
// use p2p_discovery::iterator::Iterator;
// // use p2p_identity::P2PIdentity;
// use peer::PeerStore;
// use std::{
//     sync::Arc,
//     time::{Duration, SystemTime},
// };
// use tokio::sync::Mutex;

// pub(crate) struct DialScheduler {
//     handshake_routine: HandshakeRoutine,
// }

// impl DialScheduler {
//     pub fn new(
//         disc_iterator: Arc<Iterator>,
//         // host_state: Arc<HostState>,
//         p2p_dial_interval: Option<u16>,
//     ) -> DialScheduler {
//         let min_interval = match p2p_dial_interval {
//             Some(i) => Duration::from_millis(i.into()),
//             None => Duration::from_millis(2000),
//         };

//         let handshake_routine =
//             HandshakeRoutine::new(min_interval, disc_iterator, host_state);

//         DialScheduler { handshake_routine }
//     }

//     pub fn start(&self) {
//         self.handshake_routine.run();
//     }
// }

// struct HandshakeRoutine {
//     is_running: Arc<Mutex<bool>>,
//     min_interval: Duration,
//     disc_iterator: Arc<Iterator>,
//     // host_state: Arc<HostState>,
// }

// impl HandshakeRoutine {
//     pub fn new(
//         min_interval: Duration,
//         disc_iterator: Arc<Iterator>,
//         // host_state: Arc<HostState>,
//     ) -> HandshakeRoutine {
//         let is_running = Arc::new(Mutex::new(false));

//         HandshakeRoutine {
//             disc_iterator,
//             is_running,
//             min_interval,
//             // host_state,
//         }
//     }

//     pub fn run(&self) {
//         info!("P2P handshake routine starts to run");

//         let is_running = self.is_running.clone();
//         let min_interval = self.min_interval;
//         let task_queue = self.host_state.task_queue.clone();
//         let disc_iterator = self.disc_iterator.clone();
//         let peer_store = self.host_state.peer_store.clone();
//         let host_state = self.host_state.clone();
//         let active_calls = self.host_state.handshake_active_calls.clone();

//         tokio::spawn(async move {
//             let mut is_running_lock = is_running.lock().await;
//             *is_running_lock = true;
//             std::mem::drop(is_running_lock);

//             loop {
//                 let start = SystemTime::now();

//                 // let node_val = match disc_iterator.next().await {
//                 //     Ok(n) => match n.get_value().await {
//                 //         Some(v) => v,
//                 //         None => {
//                 //             error!("Invalid node. Node is empty");
//                 //             continue;
//                 //         }
//                 //     },
//                 //     Err(err) => {
//                 //         error!(
//                 //             "P2P handshake, can't retrieve next \
//                 //             node, err: {}",
//                 //             err
//                 //         );
//                 //         continue;
//                 //     }
//                 // };

//                 // if active_calls.contains(&node_val.addr.ip).await {
//                 //     debug!(
//                 //         "Already talking with this node, ip: {}",
//                 //         node_val.addr.ip
//                 //     );

//                 //     continue;
//                 // }

//                 // TODO
//                 // if let Some(_) = peer_store.find(node_val.public_key).await {
//                 //     debug!(
//                 //         "She is already a peer, public_key: {:?}",
//                 //         node_val.public_key,
//                 //     );
//                 //     continue;
//                 // }

//                 let peer = match peer_store.reserve().await {
//                     Ok(p) => p,
//                     Err(err) => {
//                         error!("Can't reserve, err: {}", err);
//                         break;
//                     }
//                 };

//                 // let hs_init_task_params = HSInitTaskParams {
//                 //     identity: host_state.identity.clone(),
//                 //     my_rpc_port: host_state.my_rpc_port,
//                 //     my_p2p_port: host_state.my_p2p_port,
//                 //     her_ip: node_val.addr.ip,
//                 //     her_p2p_port: node_val.p2p_port,
//                 //     her_public_key: node_val.public_key,
//                 //     peer_store: peer_store.clone(),
//                 //     peer: peer,
//                 //     handshake_active_calls: active_calls.clone(),
//                 // };

//                 // match task_queue
//                 //     .push_back(Task::InitiateHandshake(hs_init_task_params))
//                 //     .await
//                 // {
//                 //     Ok(_) => (),
//                 //     Err(err) => {
//                 //         error!("Can't enqueue a task, err: {}", err);
//                 //         continue;
//                 //     }
//                 // };

//                 match start.elapsed() {
//                     Ok(d) => {
//                         if d < min_interval {
//                             let diff = min_interval - d;
//                             tokio::time::sleep(diff).await;
//                         }
//                     }
//                     Err(err) => {
//                         error!(
//                             "Calculating the time elapsed fail, err: {}",
//                             err
//                         );

//                         tokio::time::sleep(min_interval).await;
//                     }
//                 }
//             }

//             let mut is_running_lock = is_running.lock().await;
//             *is_running_lock = false;
//         });
//     }

//     pub async fn wakeup(&self) {
//         let is_running = self.is_running.lock().await;

//         if *is_running == false {
//             warn!("P2P dial scheduler routine wakes up");

//             self.run();
//         }
//     }
// }

use crate::p2p::{state::HostState, task::P2PTask};
use logger::{terr, tinfo, twarn};
use p2p_discovery::{AddrGuard, AddrsIterator};
use p2p_identity::addr::Addr;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;

const HANDSHAKE_DIAL_INTERVAL: u64 = 2000;

pub(crate) struct HandshakeDialLoop {
    pub(crate) p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) addrs_iter: Arc<AddrsIterator>,
    pub(crate) host_state: Arc<HostState>,
}

impl HandshakeDialLoop {
    pub(crate) async fn run(&self) {
        tinfo!("saksaha", "p2p", "Handshake dial loop starts looping",);

        let p2p_dial_interval = match self.p2p_dial_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(HANDSHAKE_DIAL_INTERVAL),
        };

        loop {
            println!("handshake dial loop iteration");

            let time_since = SystemTime::now();

            if let Some(addr_guard) = self.addrs_iter.next().await {
                // let addr = item.get_value();

                let task = P2PTask::InitiateHandshake {
                    addr_guard,
                    // addr,
                    host_state: self.host_state.clone(),
                };

                match self.p2p_task_queue.push_back(task).await {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "p2p",
                            "Error enqueueing a p2p handshake task, err: {}",
                            err
                        );
                    }
                }
            }

            utils_time::wait_until_min_interval(time_since, p2p_dial_interval)
                .await;
        }
    }
}
