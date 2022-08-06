// use super::{handler, NodeTask};
// use log::{debug, error};
// use sak_task_queue::TaskQueue;
// use std::{
//     sync::Arc,
//     time::{Duration, SystemTime},
// };

// const TASK_MIN_INTERVAL: u64 = 1000;

// pub(in crate::node) struct TaskRuntime {
//     pub(crate) task_queue: Arc<TaskQueue<NodeTask>>,
//     pub(crate) task_min_interval: Duration,
// }

// impl TaskRuntime {
//     pub(crate) fn new(
//         task_queue: Arc<TaskQueue<NodeTask>>,
//         disc_task_interval: Option<u16>,
//     ) -> TaskRuntime {
//         let task_min_interval = match disc_task_interval {
//             Some(i) => Duration::from_millis(i.into()),
//             None => Duration::from_millis(TASK_MIN_INTERVAL),
//         };

//         TaskRuntime {
//             task_queue,
//             task_min_interval,
//         }
//     }

//     pub(crate) async fn run(&self) {
//         let task_min_interval = &self.task_min_interval;
//         let task_queue = &self.task_queue;

//         loop {
//             let time_since = SystemTime::now();

//             let task = match task_queue.pop_front().await {
//                 Ok(t) => {
//                     debug!("Pop P2PTask - {}", t,);

//                     t
//                 }
//                 Err(err) => {
//                     error!(
//                         "Cannot handle p2p discovery task any more, \
//                                 err: {}",
//                         err,
//                     );
//                     return;
//                 }
//             };

//             handler::run(task).await;

//             sak_utils_time::wait_until_min_interval(
//                 time_since,
//                 *task_min_interval,
//             )
//             .await;
//         }
//     }
// }
