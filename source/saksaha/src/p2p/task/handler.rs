use super::task::{P2PTaskInstance, TaskResult};
use crate::p2p::task::P2PTask;
use logger::twarn;
use p2p_transport::handshake;

pub(crate) struct Handler {
    pub(crate) task_instance: P2PTaskInstance,
}

impl Handler {
    pub(crate) async fn run(&self) -> TaskResult {
        do_task(self.task_instance.clone()).await
    }
}

async fn do_task(task_instance: P2PTaskInstance) -> TaskResult {
    let task = task_instance.task;

    println!("Will do a task, {}", task);

    match &*task {
        P2PTask::InitiateHandshake { addr } => {
            match handshake::initiate_handshake(addr).await {
                Ok(_) => (),
                Err(err) => {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Error processing InitiateHandshake, discarding, \
                        err: {}",
                        err,
                    );

                    return TaskResult::Fail;
                }
            }
        }
    };

    return TaskResult::Success;

    // match &*task {
    //     DiscoveryTask::InitiateWhoAreYou { addr, disc_state } => {
    //         match whoareyou::init_who_are_you(addr.clone(), disc_state.clone())
    //             .await
    //         {
    //             Ok(_) => {
    //                 return TaskResult::Success;
    //             }
    //             Err(err) => {
    //                 return TaskResult::FailRetry { msg: err };
    //             }
    //         }
    //     }
    // };
}
