use crate::{DiscState, task::Task};
use log::{debug, error, info, warn};
use saksaha_p2p_identity::Identity;
use saksaha_task::task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub(crate) struct DialScheduler {
    revalidator: Revalidator,
}

impl DialScheduler {
    pub fn new(
        disc_state: Arc<DiscState>,
        task_queue: Arc<TaskQueue<Task>>,
    ) -> DialScheduler {
        let min_interval = Duration::from_millis(2000);
        let revalidator = Revalidator::new(disc_state, task_queue, min_interval);

        DialScheduler { revalidator }
    }

    pub fn start(&self) -> Result<(), String> {
        self.revalidator.run();

        Ok(())
    }
}

pub(crate) struct Revalidator {
    disc_state: Arc<DiscState>,
    task_queue: Arc<TaskQueue<Task>>,
    is_running: Arc<Mutex<bool>>,
    min_interval: Duration,
}

impl Revalidator {
    pub fn new(
        disc_state: Arc<DiscState>,
        task_queue: Arc<TaskQueue<Task>>,
        min_interval: Duration,
    ) -> Revalidator {
        let is_running = Arc::new(Mutex::new(false));

        Revalidator {
            is_running,
            disc_state,
            task_queue,
            min_interval,
        }
    }

    pub fn run(&self) {
        info!("Discovery dial scheduler routine starts to run");

        let is_running = self.is_running.clone();
        let min_interval = self.min_interval;

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            debug!("TODO Revalidator is currently no-op");

            loop {
                let start = SystemTime::now();

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
            warn!("Disc dial routine is not running, waking up");

            self.run();
        }
    }
}
