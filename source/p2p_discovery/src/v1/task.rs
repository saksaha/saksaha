use super::{
    address::Address,
    operations::whoareyou::{
        initiate::WhoareyouInitError, operation::WhoareyouOp,
    },
};
use log::{debug, error, warn};
use std::sync::Arc;
use task::task_queue::{TaskResult, TaskRun};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateWhoAreYou {
        whoareyou_op: Arc<WhoareyouOp>,
        addr: Address,
    },
}

pub(crate) struct TaskRunner;

impl TaskRun<Task> for TaskRunner {
    fn run(&self, task: Task) -> TaskResult {
        futures::executor::block_on(async {
            match task {
                Task::InitiateWhoAreYou { whoareyou_op, addr } => {
                    match whoareyou_op.send_who_are_you(addr).await {
                        Ok(_) => (),
                        Err(err) => {
                            let err_msg = err.to_string();

                            match err {
                                WhoareyouInitError::MyEndpoint { .. } => {
                                    return TaskResult::Fail(err_msg);
                                }
                                WhoareyouInitError::ByteConversionFail {
                                    ..
                                } => {
                                    return TaskResult::Fail(err_msg);
                                }
                                WhoareyouInitError::MessageParseFail {
                                    ..
                                } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::VerifiyingKeyFail {
                                    ..
                                } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::InvalidSignature {
                                    ..
                                } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::SendFail(_) => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::NodeReserveFail {
                                    ..
                                } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::NodeRegisterFail {
                                    ..
                                } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::TableIsFull { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::TableAddFail { .. } => {
                                    return TaskResult::FailRetriable(err_msg);
                                }
                                WhoareyouInitError::NodePutBackFail {
                                    ..
                                } => {}
                            }
                        }
                    }
                }
            };

            TaskResult::Success
        })
    }
}
