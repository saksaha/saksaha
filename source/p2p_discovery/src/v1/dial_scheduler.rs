use crate::v1::task::TaskInstance;

use super::{
    state::DiscState,
    task::{DiscoveryTask, DiscoveryTaskInstance},
};
use logger::{tinfo, twarn};
use p2p_identity::addr::Addr;
use std::{sync::Arc, time::Duration};
use task_queue::TaskQueue;

pub(crate) struct DialSchedulerArgs {
    pub(crate) disc_state: Arc<DiscState>,
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<Addr>,
    pub(crate) task_queue: Arc<TaskQueue<DiscoveryTaskInstance>>,
}

pub(crate) struct DialScheduler {
    disc_state: Arc<DiscState>,
    task_queue: Arc<TaskQueue<DiscoveryTaskInstance>>,
    dial_loop: Arc<DialLoop>,
    bootstrap_addrs: Vec<Addr>,
}

struct DialLoop {
    task_queue: Arc<TaskQueue<DiscoveryTaskInstance>>,
    disc_dial_interval: Duration,
}

impl DialScheduler {
    pub async fn init(dial_schd_args: DialSchedulerArgs) -> DialScheduler {
        let DialSchedulerArgs {
            task_queue,
            bootstrap_addrs,
            disc_dial_interval,
            disc_state,
        } = dial_schd_args;

        let disc_dial_interval = match disc_dial_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(2000),
        };

        let dial_loop = {
            let l = DialLoop {
                task_queue: task_queue.clone(),
                disc_dial_interval,
            };
            Arc::new(l)
        };

        let d = DialScheduler {
            disc_state: disc_state.clone(),
            task_queue: task_queue.clone(),
            dial_loop,
            bootstrap_addrs,
        };

        tinfo!(
            "p2p_discovery",
            "dial_schd",
            "Discovery dial scheduler is initialized. Disc dial min \
            interval: {:?}",
            disc_dial_interval,
        );

        d
    }

    async fn enqueue_bootstrap_addrs(&self, bootstrap_addrs: &Vec<Addr>) {
        let total_count = bootstrap_addrs.len();

        tinfo!(
            "p2p_discovery",
            "dial_schd",
            "Enqueueing bootstrap addrs, total count: {}",
            total_count,
        );

        for (idx, addr) in bootstrap_addrs.iter().enumerate() {
            tinfo!(
                "p2p_discovery",
                "dial_schd",
                "-- [{}/{}] enqueueing bootstrap addr, disc_endpoint: {}",
                idx + 1,
                total_count,
                addr.disc_endpoint(),
            );

            let task = {
                let t = DiscoveryTask::InitiateWhoAreYou {
                    addr: addr.clone(),
                    disc_state: self.disc_state.clone(),
                };

                TaskInstance::new(t)
            };

            match self.task_queue.push_back(task).await {
                Ok(_) => {}
                Err(err) => {
                    twarn!(
                        "p2p_discovery",
                        "dial_schd",
                        "Cannot enqueue a bootstrap addr, err: {}",
                        err,
                    );
                }
            };
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        self.enqueue_bootstrap_addrs(&self.bootstrap_addrs).await;
        self.dial_loop.run();

        Ok(())
    }
}

impl DialLoop {
    fn run(&self) {
        tinfo!(
            "p2p_discovery",
            "dial_schd",
            "Discovery dial loop currently not functional",
        );

        // loop {
        //     let start = SystemTime::now();

        //     match start.elapsed() {
        //         Ok(d) => {
        //             if d < self.min_interval {
        //                 let diff = self.min_interval - d;
        //                 tokio::time::sleep(diff).await;
        //             }
        //         }
        //         Err(err) => {
        //             terr!(
        //                 "p2p_discovery",
        //                 "Calculating the time elapsed fail, err: {}",
        //                 err
        //             );

        //             tokio::time::sleep(self.min_interval).await;
        //         }
        //     }
        // }
    }
}
