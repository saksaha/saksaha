use super::{address::Address, state::DiscState, task::DiscoveryTask};
use logger::{tdebug, terr, tinfo, twarn};
use p2p_identity::{addr::Addr, peer::UnknownPeer};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;
use tokio::sync::Mutex;

pub(crate) struct DialSchedulerArgs {
    pub(crate) disc_state: Arc<DiscState>,
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) bootstrap_addrs: Vec<Addr>,
    pub(crate) task_queue: Arc<TaskQueue<DiscoveryTask>>,
}

pub(crate) struct DialScheduler {
    disc_state: Arc<DiscState>,
    min_interval: Duration,
    is_dial_loop_running: Arc<Mutex<bool>>,
    task_queue: Arc<TaskQueue<DiscoveryTask>>,
    dial_loop: Arc<DialLoop>,
}

struct DialLoop {
    task_queue: Arc<TaskQueue<DiscoveryTask>>,
    min_interval: Duration,
}

impl DialScheduler {
    pub async fn init(dial_schd_args: DialSchedulerArgs) -> DialScheduler {
        let DialSchedulerArgs {
            task_queue,
            bootstrap_addrs,
            ..
        } = dial_schd_args;

        let min_interval = match dial_schd_args.disc_dial_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(2000),
        };

        let dial_loop = {
            let l = DialLoop {
                task_queue: task_queue.clone(),
                min_interval,
            };
            Arc::new(l)
        };

        let d = DialScheduler {
            disc_state: dial_schd_args.disc_state.clone(),
            min_interval,
            is_dial_loop_running: Arc::new(Mutex::new(false)),
            task_queue: task_queue.clone(),
            dial_loop,
        };

        tinfo!(
            "p2p_discovery",
            "dial_schd",
            "Discovery dial scheduler is initialized. Dial interval: {:?}",
            min_interval,
        );

        d.enqueue_bootstrap_addrs(bootstrap_addrs).await;

        d
    }

    async fn enqueue_bootstrap_addrs(&self, bootstrap_addrs: Vec<Addr>) {
        for addr in bootstrap_addrs {
            let task = DiscoveryTask::InitiateWhoAreYou {
                // whoareyou_op: self.whoareyou_op.clone(),
                // disc_state: self.disc_state.clone(),
                addr: addr.clone(),
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

    pub fn start(&self) -> Result<(), String> {
        let dial_loop = DialLoop {
            task_queue: self.task_queue.clone(),
            min_interval: self.min_interval,
        };

        self.dial_loop.run();

        Ok(())
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
