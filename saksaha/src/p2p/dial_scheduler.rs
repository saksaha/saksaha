use crate::p2p::ops::handshake::{self};

use super::{ops::handshake::HandshakeOp, task::Task};
use log::{debug, error, info, warn};
use saksaha_p2p_discovery::iterator::Iterator;
use saksaha_p2p_identity::Identity;
use saksaha_task::task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

pub struct DialScheduler {
    handshake_routine: HandshakeRoutine,
}

impl DialScheduler {
    pub fn new(
        task_queue: Arc<TaskQueue<Task>>,
        disc_iterator: Arc<Iterator>,
        credential: Arc<Box<dyn Identity + Send + Sync>>,
    ) -> DialScheduler {
        let min_interval = Duration::from_millis(2000);

        let handshake_routine = HandshakeRoutine::new(
            task_queue.clone(),
            min_interval,
            disc_iterator,
        );

        DialScheduler { handshake_routine }
    }

    pub fn start(&self) {
        self.handshake_routine.run();
    }
}

struct HandshakeRoutine {
    task_queue: Arc<TaskQueue<Task>>,
    is_running: Arc<Mutex<bool>>,
    min_interval: Duration,
    disc_iterator: Arc<Iterator>,
    handshake_op: Arc<HandshakeOp>,
}

impl HandshakeRoutine {
    pub fn new(
        task_queue: Arc<TaskQueue<Task>>,
        min_interval: Duration,
        disc_iterator: Arc<Iterator>,
    ) -> HandshakeRoutine {
        let is_running = Arc::new(Mutex::new(false));

        let handshake_op = Arc::new(HandshakeOp::new());

        HandshakeRoutine {
            task_queue,
            disc_iterator,
            is_running,
            min_interval,
            handshake_op,
        }
    }

    pub fn run(&self) {
        info!("P2P dial scheduler routine starts to run");

        let is_running = self.is_running.clone();
        let min_interval = self.min_interval;
        let task_queue = self.task_queue.clone();
        let disc_iterator = self.disc_iterator.clone();
        let handshake_op = self.handshake_op.clone();
        // let credential = self.cre

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let start = SystemTime::now();

                let node_val = match disc_iterator.next().await {
                    Ok(n) => match n.get_value().await {
                        Some(v) => v,
                        None => {
                            error!("Can't retrieve next node. Node is empty");
                            continue;
                        }
                    },
                    Err(err) => {
                        error!(
                            "Discovery iterator cannot retrieve next \
                            node, err: {}",
                            err
                        );
                        continue;
                    }
                };

                // task_queue.push(Task::InitiateHandshake {
                //     endpoint: node_val.addr.endpoint(),
                //     my_public_key: ,
                //     handshake_op: ,
                // }).await;

                // task_queue.push(Task::SendHandshakeSyn {
                //     endpoint
                //     handshake_op,
                // });

                match start.elapsed() {
                    Ok(d) => {
                        if d < min_interval {
                            let diff = min_interval - d;
                            tokio::time::sleep(diff).await;
                        }
                    }
                    Err(err) => {
                        error!(
                            "Calculating the time elapsed fail, err: {}",
                            err
                        );

                        tokio::time::sleep(min_interval).await;
                    }
                }
            }

            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = false;
        });
    }

    pub async fn wakeup(&self) {
        let is_running = self.is_running.lock().await;

        if *is_running == false {
            warn!("P2P dial scheduler routine wakes up");

            self.run();
        }
    }
}
