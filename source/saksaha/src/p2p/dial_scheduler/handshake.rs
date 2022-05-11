use crate::p2p::{state::HostState, task::P2PTask};
use logger::{terr, tinfo};
use p2p_discovery::AddrsIterator;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;

const HANDSHAKE_DIAL_INTERVAL: u64 = 2000;
const HANDSHAKE_ENQUEUE_DELAY_WHEN_SMALLER_PUBLIC_KEY: u64 = 4;

pub(crate) struct HandshakeDialLoop {
    pub(crate) p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) addrs_iter: Arc<AddrsIterator>,
    pub(crate) host_state: Arc<HostState>,
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
                Ok(addr_guard) => {
                    let known_addr =
                        match addr_guard.get_known_addr().await {
                            Ok(a) => a,
                            Err(err) => {
                                terr!(
                                "saksaha", 
                                "p2p", 
                                "Addr table has invalid entry (not known), \
                                err: {}", err);
                                continue;
                            }
                        };

                    let my_public_key_str =
                        &self.host_state.p2p_identity.public_key_str;
                    let her_public_key_str = &known_addr.public_key_str;
                    let is_my_public_key_greater_than_hers =
                        my_public_key_str > her_public_key_str;

                    let task = P2PTask::InitiateHandshake {
                        addr_guard,
                        host_state: self.host_state.clone(),
                    };
                    let p2p_task_queue = self.p2p_task_queue.clone();

                    if is_my_public_key_greater_than_hers {
                        enqueue_task(p2p_task_queue, task).await;
                    } else {
                        // enqueue_task(p2p_task_queue, task).await;
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
                    terr!(
                        "saksaha",
                        "p2p",
                        "Error (fatal) getting next addr node, err: {}",
                        err
                    );
                }
            };

            utils_time::wait_until_min_interval(time_since, p2p_dial_interval)
                .await;
        }
    }
}

async fn enqueue_task(task_queue: Arc<TaskQueue<P2PTask>>, task: P2PTask) {
    match task_queue.push_back(task).await {
        Ok(_) => {}
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
