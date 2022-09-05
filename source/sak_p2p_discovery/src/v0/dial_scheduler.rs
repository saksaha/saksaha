use super::task::DiscoveryTask;
use log::{info, warn};
use sak_p2p_addr::UnknownAddr;
use sak_task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const DISC_DIAL_INTERVAL: u64 = 2000;

pub(crate) struct DialScheduler {
    disc_task_queue: Arc<TaskQueue<DiscoveryTask>>,
    bootstrap_addrs: Vec<UnknownAddr>,
}

pub(crate) struct DialSchedulerArgs {
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
    pub(crate) disc_task_queue: Arc<TaskQueue<DiscoveryTask>>,
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

        let d = DialScheduler {
            disc_task_queue: disc_task_queue.clone(),
            bootstrap_addrs,
        };

        info!(
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

        info!("Enqueueing bootstrap addrs, total count: {}", total_count,);

        for (idx, addr) in bootstrap_addrs.iter().enumerate() {
            info!(
                "-- [{}/{}] enqueueing bootstrap addr, disc_endpoint: {}",
                idx + 1,
                total_count,
                addr.disc_endpoint(),
            );

            let task = DiscoveryTask::InitiateWhoAreYou { addr: addr.clone() };

            match self.disc_task_queue.push_back(task).await {
                Ok(_) => {}
                Err(err) => {
                    warn!("Cannot enqueue a bootstrap addr, err: {}", err,);
                }
            };
        }
    }

    // async fn enqueue_new_addrs(&self, new_addr: &UnknownAddr) {
    //     //
    //     let task = DiscoveryTask::InitiateWhoAreYou {
    //         addr: new_addr.clone(),
    //     };

    //     match self.disc_task_queue.push_back(task).await {
    //         Ok(_) => {}
    //         Err(err) => {
    //             warn!(
    //                 "Cannot enqueue a new addr, addr: {:?}, err: {}",
    //                 new_addr, err
    //             );
    //         }
    //     };
    // }

    pub async fn run(&self) {
        self.enqueue_bootstrap_addrs(&self.bootstrap_addrs).await;
    }
}
