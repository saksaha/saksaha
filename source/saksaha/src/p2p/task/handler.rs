use super::task::{P2PTaskInstance, TaskResult};
use crate::p2p::task::P2PTask;
use logger::twarn;
use p2p_transport::handshake::{self, HandshakeInitArgs};

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
        P2PTask::InitiateHandshake { addr, host_state } => {
            let handshake_init_args = HandshakeInitArgs {
                addr: addr.clone(),
                p2p_port: host_state.p2p_port,
            };

            match handshake::initiate_handshake(handshake_init_args).await {
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
