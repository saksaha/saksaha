use super::{
    address::Address,
    ops::whoareyou::{initiator::WhoAreYouInitError, WhoAreYouOperator},
};
use log::{debug, error, warn};
use saksaha_task::task_queue::{TaskResult, TaskRun};
use std::{sync::Arc};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateWhoAreYou {
        way_operator: Arc<WhoAreYouOperator>,
        addr: Address,
    },
}

pub struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                Task::InitiateWhoAreYou { way_operator, addr } => {
                    match way_operator.initiator.send_who_are_you(addr).await {
                        Ok(_) => (),
                        Err(err) => {
                            let err_msg = err.to_string();

                            match err {
                                WhoAreYouInitError::MyEndpoint { .. } => {
                                    return TaskResult::Fail(err_msg);
                                }
                                WhoAreYouInitError::ByteConversionFail {
                                    ..
                                } => {
                                    return TaskResult::Fail(err_msg);
                                }
                                WhoAreYouInitError::MessageParseFail { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::VerifiyingKeyFail {
                                    ..
                                } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::InvalidSignature { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::SendFail(_) => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::NodeReserveFail { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::NodeRegisterFail { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::TableIsFull { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoAreYouInitError::TableAddFail { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                            }
                        }
                    }
                }
            };

            TaskResult::Success
        })
    }
}
