use std::{alloc::dealloc, sync::Arc};
use once_cell::sync::OnceCell;
use logger::log;
use crate::node::Node;

static INSTANCE: OnceCell<Process> = OnceCell::new();

pub struct Process {
    node: Box<Node>,
}

impl Process {
    pub fn init(node: Box<Node>) {
        let p = Process {
            node,
        };

        match INSTANCE.set(p) {
            Ok(_) => (),
            Err(err) => {
                log!(DEBUG, "Cannot initialize process\n");

                std::process::exit(1);
            }
        }
    }

    pub fn shutdown() {
        let process = match INSTANCE.get() {
            Some(p) => p,
            None => {
                log!(DEBUG, "Process is not initialized. Consider calling \
                    Process:init()\n");

                std::process::exit(1);
            },
        };

        log!(DEBUG, "Preparing to shutdown process\n");

        process.node.persist_state();

        std::process::exit(1);
    }

}
