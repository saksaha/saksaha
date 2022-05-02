use crate::v1::instr::whoareyou;

use super::{DiscoveryTask, DiscoveryTaskInstance, TaskResult};

pub(crate) struct Handler {
    pub(crate) task_instance: DiscoveryTaskInstance,
}

impl Handler {
    pub(crate) async fn run(&self) -> TaskResult {
        do_task(self.task_instance.clone()).await
    }
}

async fn do_task(task_instance: DiscoveryTaskInstance) -> TaskResult {
    let task = task_instance.task;

    match &*task {
        DiscoveryTask::InitiateWhoAreYou { addr, disc_state } => {
            match whoareyou::init_who_are_you(addr.clone(), disc_state.clone())
                .await
            {
                Ok(_) => {
                    return TaskResult::Success;
                }
                Err(err) => {
                    return TaskResult::FailRetry { msg: err };
                }
            }
        }
    };
}
