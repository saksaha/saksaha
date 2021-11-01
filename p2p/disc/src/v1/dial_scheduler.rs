use crate::{v1::task_queue::TaskQueue, DiscState};
use log::{debug, error, info, warn};
use saksaha_p2p_identity::Identity;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub struct DialScheduler {
    routine: Routine,
}

impl DialScheduler {
    pub fn new(
        disc_state: Arc<DiscState>,
        task_queue: Arc<TaskQueue>,
    ) -> DialScheduler {
        let routine = Routine::new(disc_state, task_queue);

        DialScheduler { routine }
    }

    pub fn start(&self) -> Result<(), String> {
        self.routine.run();

        Ok(())
    }
}

pub struct Routine {
    disc_state: Arc<DiscState>,
    task_queue: Arc<TaskQueue>,
    is_running: Arc<Mutex<bool>>,
}

impl Routine {
    pub fn new(
        disc_state: Arc<DiscState>,
        task_queue: Arc<TaskQueue>,
    ) -> Routine {
        let is_running = Arc::new(Mutex::new(false));

        Routine {
            is_running,
            disc_state,
            task_queue,
        }
    }

    pub fn run(&self) {
        info!("Discovery dial scheduler routine starts to run");

        let is_running = self.is_running.clone();

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let start = SystemTime::now();

                tokio::time::sleep(Duration::from_millis(100)).await;

                match start.elapsed() {
                    Ok(_) => (),
                    Err(err) => {
                        warn!("Error sleeping the duration, err: {}", err);
                    }
                };
            }

            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = false;
        });
    }

    pub async fn wakeup(&self) {
        let is_running = self.is_running.lock().await;

        if *is_running == false {
            warn!("Disc dial routine is not running, waking up");

            self.run();
        }
    }
}
