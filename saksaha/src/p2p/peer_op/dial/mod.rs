mod routine;

use std::{sync::Arc, time::Duration};
use crate::node::task_manager::TaskManager;
use logger::log;
use tokio::sync::mpsc::Sender;
use routine::Routine;

pub struct Dial {
    task_mng: Arc<TaskManager>,
    disc_wakeup_tx: Arc<Sender<usize>>,
}

impl Dial {
    pub fn new(
        task_mng: Arc<TaskManager>,
        disc_wakeup_tx: Arc<Sender<usize>>,
    ) -> Dial {
        Dial { task_mng, disc_wakeup_tx }
    }

    pub async fn start(self) {
        log!(DEBUG, "Start peer op dialing\n");

        let routine = Routine::new();

        // tokio::time::sleep(Duration::from_millis(4000)).await;

        // println!("peer op dial woke up");

        // match self.dial_wakeup_tx.send(0).await {
        //     Ok(_) => {
        //         println!("peer op dial start sent!");
        //     },
        //     Err(err) => {
        //         println!("peer op dial start send fail, err: {}", err);
        //     }
        // };
    }
}
