use super::task::DiscoveryTask;
use logger::{tinfo, twarn};
use p2p_addr::UnknownAddr;
use std::{sync::Arc, time::Duration};
use task_queue::TaskQueue;

const DISC_DIAL_INTERVAL: u64 = 2000;

pub(crate) struct DialSchedulerArgs {
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
    pub(crate) disc_task_queue: Arc<TaskQueue<DiscoveryTask>>,
}

pub(crate) struct DialScheduler {
    disc_task_queue: Arc<TaskQueue<DiscoveryTask>>,
    dial_loop: Arc<DialLoop>,
    bootstrap_addrs: Vec<UnknownAddr>,
}

struct DialLoop {
    disc_task_queue: Arc<TaskQueue<DiscoveryTask>>,
    disc_dial_interval: Duration,
}

impl DialScheduler {
    pub fn init(dial_schd_args: DialSchedulerArgs) -> DialScheduler {
        let DialSchedulerArgs {
            disc_task_queue,
            bootstrap_addrs,
            disc_dial_interval,
        } = dial_schd_args;

        let disc_dial_interval = match disc_dial_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(DISC_DIAL_INTERVAL),
        };

        let dial_loop = {
            let l = DialLoop {
                disc_task_queue: disc_task_queue.clone(),
                disc_dial_interval,
            };
            Arc::new(l)
        };

        let d = DialScheduler {
            disc_task_queue: disc_task_queue.clone(),
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

    async fn enqueue_bootstrap_addrs(
        &self,
        bootstrap_addrs: &Vec<UnknownAddr>,
    ) {
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

            let task = DiscoveryTask::InitiateWhoAreYou { addr: addr.clone() };

            match self.disc_task_queue.push_back(task).await {
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

    pub async fn run(&self) {
        self.enqueue_bootstrap_addrs(&self.bootstrap_addrs).await;

        self.dial_loop.run().await;
    }
}

impl DialLoop {
    async fn run(&self) {
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
