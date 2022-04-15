use super::{address::Address, state::DiscState, task::DiscoveryTask};
use logger::{tdebug, terr, tinfo, twarn};
use p2p_identity::peer::UnknownPeer;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;
use tokio::sync::Mutex;

pub(crate) struct DialScheduler {
    disc_state: Arc<DiscState>,
    dial_routine: DialRoutine,
    // revalidate_routine: RevalidateRoutine,
    // whoareyou_op: Arc<WhoareyouOp>,
}

struct DialRoutine {
    is_running: Arc<Mutex<bool>>,
    min_interval: Duration,
}

impl DialScheduler {
    pub async fn init(
        disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        // bootstrap_urls: Option<Vec<String>>,
        // default_bootstrap_urls: &str,
        // task_queue: Arc<TaskQueue<DiscoveryTask>>,
        unknown_peers: Vec<UnknownPeer>,
    ) -> DialScheduler {
        let task_queue = TaskQueue::<DiscoveryTask>::new(20);

        enqueue_initial_tasks(task_queue, unknown_peers).await;

        let min_interval = Duration::from_millis(2000);

        // let revalidate_routine =
        //     RevalidateRoutine::new(disc_state.clone(), min_interval);

        let dial_routine = DialRoutine::new(min_interval);

        let d = DialScheduler {
            // revalidate_routine,
            disc_state: disc_state.clone(),
            dial_routine,
            // whoareyou_op,
        };

        d
    }

    pub fn start(&self) -> Result<(), String> {
        // self.revalidate_routine.run();

        self.dial_routine.run();

        Ok(())
    }
}

impl DialRoutine {
    fn new(min_interval: Duration) -> DialRoutine {
        DialRoutine {
            min_interval,
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    async fn run(&self) {
        tinfo!(
            "p2p_discovery",
            "dial_schd",
            "Discovery dial scheduler starts to run"
        );

        let min_interval = self.min_interval;
        let mut is_running = self.is_running.lock().await;
        *is_running = true;
        drop(is_running);

        let routine_process = tokio::spawn(async move {
            // let mut is_running_lock = is_running.lock().await;
            // *is_running_lock = true;
            // drop(is_running_lock);

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
                        terr!(
                            "p2p_discovery",
                            "Calculating the time elapsed fail, err: {}",
                            err
                        );

                        tokio::time::sleep(min_interval).await;
                    }
                }
            }
        });

        match routine_process.await {
            Ok(_) => {
                terr!(
                    "p2p_discovery",
                    "dial_schd",
                    "dial routine has ended, which is likely unusual",
                );
            }
            Err(err) => {
                terr!(
                    "p2p_discovery",
                    "dial_schd",
                    "dial routine has ended with an error, {}",
                    err,
                );
            }
        };

        let mut is_running = self.is_running.lock().await;
        *is_running = true;
    }
}

async fn enqueue_initial_tasks(
    task_queue: TaskQueue<DiscoveryTask>,
    unknown_peers: Vec<UnknownPeer>,
) {
    for unknown_peer in unknown_peers {
        let task = DiscoveryTask::InitiateWhoAreYou {
            // whoareyou_op: self.whoareyou_op.clone(),
            // disc_state: self.disc_state.clone(),
            // addr,
            unknown_peer,
        };

        match task_queue.push_back(task).await {
            Ok(_) => {}
            Err(err) => {
                twarn!(
                    "p2p_discovery",
                    "dial_schd",
                    "Couldn't enque new task, err: {}",
                    err
                );
            }
        };
    }
}

// pub(crate) struct RevalidateRoutine {
//     _disc_state: Arc<DiscState>,
//     is_running: Arc<Mutex<bool>>,
//     min_interval: Duration,
// }

// impl RevalidateRoutine {
//     pub fn new(
//         disc_state: Arc<DiscState>,
//         min_interval: Duration,
//     ) -> RevalidateRoutine {
//         let is_running = Arc::new(Mutex::new(false));

//         RevalidateRoutine {
//             is_running,
//             _disc_state: disc_state,
//             min_interval,
//         }
//     }

//     pub fn run(&self) {
//         tinfo!("p2p_discovery", "Dial scheduler starts to run");

//         let is_running = self.is_running.clone();
//         let min_interval = self.min_interval;

//         tokio::spawn(async move {
//             let mut is_running_lock = is_running.lock().await;
//             *is_running_lock = true;
//             std::mem::drop(is_running_lock);

//             loop {
//                 let start = SystemTime::now();

//                 tdebug!(
//                     "p2p_discovery",
//                     "TODO Discovery revalidator is currently no-op"
//                 );

//                 match start.elapsed() {
//                     Ok(d) => {
//                         if d < min_interval {
//                             let diff = min_interval - d;
//                             tokio::time::sleep(diff).await;
//                         }
//                     }
//                     Err(err) => {
//                         terr!(
//                             "p2p_discovery",
//                             "Calculating the time elapsed fail, err: {}",
//                             err
//                         );

//                         tokio::time::sleep(min_interval).await;
//                     }
//                 }
//             }

//             let mut is_running_lock = is_running.lock().await;
//             *is_running_lock = false;
//         });
//     }

//     pub async fn wakeup(&self) {
//         let is_running = self.is_running.lock().await;

//         if *is_running == false {
//             twarn!("p2p_discovery", "Dial routine is not running, waking up");

//             self.run();
//         }
//     }
// }
