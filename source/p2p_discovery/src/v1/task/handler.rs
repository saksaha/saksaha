use super::DiscoveryTask;

pub(crate) struct Handler {
    pub(crate) task: DiscoveryTask,
}

impl Handler {
    pub(crate) async fn run(&self) {
        do_task(self.task.clone()).await;
    }
}

async fn do_task(task: DiscoveryTask) {
    println!("{:?}", task);

    match task {
        DiscoveryTask::InitiateWhoAreYou { addr } => {}
    };
}
