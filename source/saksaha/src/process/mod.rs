// use crate::node::Node;
// use log::{error, info};
// use once_cell::sync::OnceCell;
// use std::sync::Arc;

// static INSTANCE: OnceCell<Process> = OnceCell::new();

// pub struct Process {
//     node: Arc<Node>,
// }

// impl Process {
//     pub fn init(node: Arc<Node>) {
//         let p = Process { node };

//         match INSTANCE.set(p) {
//             Ok(_) => (),
//             Err(err) => {
//                 error!("Cannot initialize process");

//                 std::process::exit(1);
//             }
//         }
//     }

//     pub fn shutdown() {
//         let process = match INSTANCE.get() {
//             Some(p) => p,
//             None => {
//                 error!(
//                     "Process is not initialized. Consider calling \
//                     Process:init() at the launch of the program"
//                 );

//                 std::process::exit(1);
//             }
//         };

//         info!("Preparing to shutdown process");

//         process.node.persist_state();

//         std::process::exit(1);
//     }
// }
