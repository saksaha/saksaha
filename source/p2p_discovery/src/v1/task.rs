use crate::state::DiscState;

use super::{
    address::Address,
    operations::whoareyou::{self, initiate::WhoareyouInitError},
};
use log::{debug, error, warn};
use std::{pin::Pin, sync::Arc};
use task::task_queue::{TaskResult, TaskRun, TaskQueue};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateWhoAreYou {
        // disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        addr: Address,
    },
}

pub(crate) struct DiscTaskRunner {
    disc_state: Arc<DiscState>,
}

impl DiscTaskRunner {
    pub fn new(task_queue: Arc<TaskQueue<Task>>) -> DiscTaskRunner {
        return Dis
    }
}

impl TaskRun<Task> for DiscTaskRunner {
    fn run<'a>(
        &'a self,
        task: Task,
    ) -> Pin<Box<dyn std::future::Future<Output = TaskResult> + Send + 'a>>
// where
    //     Self: Sync + 'a,
    {
        async fn run(_self: &DiscTaskRunner, task: Task) -> TaskResult {
            match task {
                Task::InitiateWhoAreYou { addr } => {
                    let disc_state = _self.disc_state.clone();
                    match whoareyou::initiate::send_who_are_you(
                        disc_state, addr,
                    )
                    .await
                    {
                        Ok(_) => TaskResult::Success,
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
                                } => {
                                    return TaskResult::Fail(err_msg);
                                }
                            }
                        }
                    }
                }
            }
        }

        Box::pin(run(self, task))
    }
    // fn run<'a>(
    //     &self,
    //     task: Task,
    // ) -> Box<dyn std::future::Future<Output = ()>> where Self: 'a
    // {
    //     // async fn async_fn(_self: &TaskRunner, task: Task) {

    //     // }

    //     // Box::pin(async_fn(self, task))
    // }

    // fn run(&self, task: Task) -> TaskResult {
    //     futures::executor::block_on(async {
    //         match task {
    //             Task::InitiateWhoAreYou { disc_state, addr } => {
    //                 match whoareyou::initiate::send_who_are_you(
    //                     disc_state, addr,
    //                 )
    //                 .await
    //                 {
    //                     Ok(_) => (),
    //                     Err(err) => {
    //                         let err_msg = err.to_string();

    //                         match err {
    //                             WhoareyouInitError::MyEndpoint { .. } => {
    //                                 return TaskResult::Fail(err_msg);
    //                             }
    //                             WhoareyouInitError::ByteConversionFail {
    //                                 ..
    //                             } => {
    //                                 return TaskResult::Fail(err_msg);
    //                             }
    //                             WhoareyouInitError::MessageParseFail {
    //                                 ..
    //                             } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::VerifiyingKeyFail {
    //                                 ..
    //                             } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::InvalidSignature {
    //                                 ..
    //                             } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::SendFail(_) => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::NodeReserveFail {
    //                                 ..
    //                             } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::NodeRegisterFail {
    //                                 ..
    //                             } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::TableIsFull { .. } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::TableAddFail { .. } => {
    //                                 return TaskResult::FailRetriable(err_msg);
    //                             }
    //                             WhoareyouInitError::NodePutBackFail {
    //                                 ..
    //                             } => {}
    //                         }
    //                     }
    //                 }
    //             }
    //         };

    //         TaskResult::Success
    //     })
    // }
}
