use super::ops::whoareyou::WhoareyouOp;
use crate::{address::Address, state::DiscState, task::Task};
use log::{debug, error, info, warn};
use task::task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

pub(crate) struct DialScheduler {
    disc_state: Arc<DiscState>,
    revalidate_routine: RevalidateRoutine,
    whoareyou_op: Arc<WhoareyouOp>,
}

impl DialScheduler {
    pub async fn init(
        disc_state: Arc<DiscState>,
        whoareyou_op: Arc<WhoareyouOp>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: String,
    ) -> DialScheduler {
        let min_interval = Duration::from_millis(2000);

        let revalidate_routine = RevalidateRoutine::new(
            disc_state.clone(),
            min_interval,
        );

        let d = DialScheduler {
            revalidate_routine,
            disc_state: disc_state.clone(),
            whoareyou_op,
        };

        d.enqueue_initial_tasks(bootstrap_urls, default_bootstrap_urls)
            .await;

        d
    }

    pub fn start(&self) -> Result<(), String> {
        self.revalidate_routine.run();

        Ok(())
    }

    pub async fn enqueue_initial_tasks(
        &self,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: String,
    ) {
        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => Vec::new(),
        };

        let default_bootstrap_urls: Vec<String> = default_bootstrap_urls
            .lines()
            .map(|l| l.to_string())
            .collect();

        let urls = [bootstrap_urls, default_bootstrap_urls].concat();

        info!("*********************************************************");
        info!("* Discovery table bootstrapped");

        let count = {
            let mut cnt = 0;
            for url in urls {
                let addr = match Address::parse(url.clone()) {
                    Ok(n) => {
                        cnt += 1;
                        n
                    }
                    Err(err) => {
                        warn!(
                            "Discarding url failed to parse, url: {}, \
                            err: {:?}",
                            url.clone(),
                            err,
                        );

                        continue;
                    }
                };

                info!("* [{}] {}", cnt, addr.short_url());

                let task = Task::InitiateWhoAreYou {
                    whoareyou_op: self.whoareyou_op.clone(),
                    addr,
                };

                match self.disc_state.task_queue.push(task).await {
                    Ok(_) => (),
                    Err(err) => {
                        warn!("Couldn't enque new task, err: {}", err);
                    }
                };
            }
            cnt
        };

        info!("* bootstrapped node count: {}", count);
        info!("*********************************************************");
    }
}

pub(crate) struct RevalidateRoutine {
    disc_state: Arc<DiscState>,
    is_running: Arc<Mutex<bool>>,
    min_interval: Duration,
}

impl RevalidateRoutine {
    pub fn new(
        disc_state: Arc<DiscState>,
        min_interval: Duration,
    ) -> RevalidateRoutine {
        let is_running = Arc::new(Mutex::new(false));

        RevalidateRoutine {
            is_running,
            disc_state,
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

            loop {
                let start = SystemTime::now();

                debug!("TODO Discovery revalidator is currently no-op");

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
                return;
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
