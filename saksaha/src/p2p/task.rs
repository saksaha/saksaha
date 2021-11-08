use log::{debug, error, warn};
use saksaha_p2p_identity::PUBLIC_KEY_LEN;
use saksaha_p2p_transport::TransportFactory;
use saksaha_task::task_queue::{TaskResult, TaskRun};
use std::sync::Arc;

#[derive(Clone)]
pub(crate) enum Task {
    InitiateHandshake {
        ip: String,
        p2p_port: u16,
        transport_factory: Arc<TransportFactory>,
    },
}

pub(crate) struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                Task::InitiateHandshake {
                    ip,
                    p2p_port,
                    transport_factory,
                } => {
                    match transport_factory.initiate_handshake(
                        ip,
                        p2p_port,
                    ).await {
                        Ok(_) => (),
                        Err(err) => {
                            let err_msg = err.to_string();
                        }
                    };
                    // match handshake_op.initiate.send_handshake_syn(
                    //     ip,
                    //     p2p_port,
                    //     my_public_key,
                    // ).await {
                    //     Ok(_) => (),
                    //     Err(err) => {
                    //         let err_msg = err.to_string();

                    //         return TaskResult::FailRetriable(err_msg);
                    //     }
                    // };
                }
            };

            TaskResult::Success
        })
    }
}
