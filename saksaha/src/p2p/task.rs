use log::{debug, error, warn};
use saksaha_task::task_queue::{TaskResult, TaskRun};
use std::{sync::Arc};

#[derive(Clone)]
pub(crate) enum Task {
    // InitiateWhoAreYou {
    //     way_operator: Arc<WhoAreYouOperator>,
    //     addr: Address,
    // },
}

pub struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                _ => (),
            };

            TaskResult::Success
        })
    }
}
