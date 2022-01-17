use crate::state::DiscState;

use super::{
    address::Address,
    operations::whoareyou::{self, initiate::WhoareyouInitError},
};
use log::{debug, error, warn};
use std::{pin::Pin, sync::Arc};
use task::task_queue::{TaskResult, TaskRun};

#[derive(Clone)]
pub(crate) enum Task {
    InitiateWhoAreYou {
        disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        addr: Address,
    },
}

pub(crate) struct TaskRunner;

trait Advertisement {
    fn run<'a>(
        &'a self,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>>;
    // where
    //     Self: Sync + 'a;
}

struct AutoplayingVideo;

impl Advertisement for AutoplayingVideo {
    fn run<'a>(
        &'a self,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>>
    // where
    //     Self: Sync + 'a,
    {
        async fn run(_self: &AutoplayingVideo) {
            /* the original method body */
        }

        Box::pin(run(self))
    }
}

impl TaskRun<Task> for TaskRunner {
    fn run<'a>(
        &'a self,
    ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>>
    // where
    //     Self: Sync + 'a,
    {
        async fn run(_self: &TaskRunner) {
            /* the original method body */
        }

        Box::pin(run(self))
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
