use crate::p2p::task::P2PTask;
use sak_logger::error;
use sak_p2p_discovery::AddrsIterator;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use sak_task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const HANDSHAKE_DIAL_INTERVAL: u64 = 2000;
const HANDSHAKE_ENQUEUE_DELAY_WHEN_SMALLER_PUBLIC_KEY: u64 = 4;

pub(crate) struct HandshakeDialLoop {
    pub(crate) p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) addrs_iter: AddrsIterator,
    pub(crate) identity: Arc<Identity>,
    pub(crate) peer_table: Arc<PeerTable>,
}

impl HandshakeDialLoop {
    pub(crate) async fn run(&self) {
        let p2p_dial_interval = match self.p2p_dial_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(HANDSHAKE_DIAL_INTERVAL),
        };

        loop {
            let time_since = SystemTime::now();

            match self.addrs_iter.next().await {
                Ok(addr) => {
                    let known_addr = &addr.known_addr;

                    let my_public_key_str =
                        &self.identity.credential.public_key_str;
                    let her_public_key_str = &known_addr.public_key_str;

                    let is_my_public_key_greater_than_hers =
                        my_public_key_str > her_public_key_str;

                    let task = P2PTask::InitiateHandshake {
                        addr,
                        identity: self.identity.clone(),
                        peer_table: self.peer_table.clone(),
                    };

                    let p2p_task_queue = self.p2p_task_queue.clone();

                    if is_my_public_key_greater_than_hers {
                        enqueue_task(p2p_task_queue, task).await;
                    } else {
                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_secs(
                                HANDSHAKE_ENQUEUE_DELAY_WHEN_SMALLER_PUBLIC_KEY,
                            ))
                            .await;

                            enqueue_task(p2p_task_queue, task).await;
                        });
                    }
                }
                Err(err) => {
                    error!(
                        "Error (fatal) getting next addr node, err: {}",
                        err
                    );
                }
            };

            sak_utils_time::wait_until_min_interval(
                time_since,
                p2p_dial_interval,
            )
            .await;
        }
    }
}

async fn enqueue_task(task_queue: Arc<TaskQueue<P2PTask>>, task: P2PTask) {
    match task_queue.push_back(task).await {
        Ok(_) => {}
        Err(err) => {
            error!("Error enqueueing a p2p handshake task, err: {}", err);
        }
    }
}
