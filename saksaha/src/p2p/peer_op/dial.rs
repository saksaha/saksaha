use std::sync::Arc;

use logger::log;
use crate::{node::task_manager::TaskManager};

pub struct Dial {
    task_mng: Arc<TaskManager>,
}

impl Dial {
    pub fn new(task_mng: Arc<TaskManager>) -> Dial {
        Dial { task_mng }
    }

    pub async fn start_dialing(self) {
        log!(DEBUG, "start p2p dialing\n");
    }
}
