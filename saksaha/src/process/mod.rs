use crate::client::Client;
use logger::log;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static INSTANCE: OnceCell<Process> = OnceCell::new();

pub struct Process {
    node: Arc<Client>,
}

impl Process {
    pub fn init(node: Arc<Client>) {
        let p = Process { node };

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
                log!(
                    DEBUG,
                    "Process is not initialized. Consider calling \
                    Process:init() at the launch of the program\n"
                );

                std::process::exit(1);
            }
        };

        log!(DEBUG, "Preparing to shutdown process\n");

        process.node.persist_state();

        std::process::exit(1);
    }
}
