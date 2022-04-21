mod handler;

pub use handler::*;

use super::address::Address;
use crate::state::DiscState;
use log::{debug, error, warn};
use p2p_identity::{addr::Addr, peer::UnknownPeer};
use std::{pin::Pin, sync::Arc};
use task_queue::TaskQueue;

#[derive(Clone, Debug)]
pub(crate) enum DiscoveryTask {
    InitiateWhoAreYou {
        // disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        addr: Addr,
        // unknown_peer: UnknownPeer,
    },
}

// pub(crate) struct DiscTaskHandler {
//     pub(crate) disc_state: Arc<DiscState>,
//     // task_queue: Arc<TaskQueue<Task>>,
// }

// impl DiscTaskHandler {
//     pub fn new(
//         disc_state: Arc<DiscState>,
//         // task_queue: Arc<TaskQueue<Task>>,
//     ) -> DiscTaskHandler {
//         return DiscTaskRunner {
//             disc_state,
//             // task_queue,
//         };
//     }
// }

// impl TaskHandle<DiscoveryTask> for DiscTaskHandler {
//     fn handle_task<'a>(
//         &'a self,
//         task: DiscoveryTask,
//     ) -> Pin<Box<dyn std::future::Future<Output = TaskResult> + Send + 'a>>
//     {
//         async fn run(
//             _self: &DiscTaskHandler,
//             task: DiscoveryTask,
//         ) -> TaskResult {
//             match task {
//                 DiscoveryTask::InitiateWhoAreYou { unknown_peer } => {
//                     let disc_state = _self.disc_state.clone();
//                     match whoareyou::initiate::send_who_are_you(
//                         disc_state,
//                         unknown_peer,
//                     )
//                     .await
//                     {
//                         Ok(_) => TaskResult::Success,
//                         Err(err) => {
//                             let err_msg = err.to_string();

//                             match err {
//                                 WhoareyouInitError::MyEndpoint { .. } => {
//                                     return TaskResult::Fail(err_msg);
//                                 }
//                                 WhoareyouInitError::ByteConversionFail {
//                                     ..
//                                 } => {
//                                     return TaskResult::Fail(err_msg);
//                                 }
//                                 WhoareyouInitError::MessageParseFail {
//                                     ..
//                                 } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::VerifiyingKeyFail {
//                                     ..
//                                 } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::InvalidSignature {
//                                     ..
//                                 } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::SendFail(_) => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::NodeReserveFail {
//                                     ..
//                                 } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::NodeRegisterFail {
//                                     ..
//                                 } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::TableIsFull { .. } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::TableAddFail { .. } => {
//                                     return TaskResult::FailRetriable(err_msg);
//                                 }
//                                 WhoareyouInitError::NodePutBackFail {
//                                     ..
//                                 } => {
//                                     return TaskResult::Fail(err_msg);
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         Box::pin(run(self, task))
//     }
// }
