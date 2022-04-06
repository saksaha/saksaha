use super::{System, SystemInner};
use logger::{terr, tinfo};
use once_cell::sync::OnceCell;
use std::sync::Arc;

static INSTANCE: OnceCell<Process> = OnceCell::new();

pub(crate) struct Process {
    // shutdownable: Arc<dyn Shutdown + Sync + Send>,
    system: Arc<Inner>,
}

impl Process {
    pub fn init(system_inner: Arc<Inner>) {
        let p = Process { system };

        match INSTANCE.set(p) {
            Ok(_) => {
                tinfo!(
                    "saksaha",
                    "system",
                    "System is attached to a singleton, umbrella process"
                );
            }
            Err(_) => {
                terr!("saksaha", "system", "Cannot initialize process");

                std::process::exit(1);
            }
        }
    }

    pub fn shutdown() {
        let process = match INSTANCE.get() {
            Some(p) => p,
            None => {
                terr!(
                    "saksaha",
                    "system",
                    "Process is not initialized. Consider calling \
                    Process:init() at the launch of the program"
                );

                std::process::exit(1);
            }
        };

        tinfo!("saksaha", "system", "Calling shutdown callback");

        // process.system.shutdown();

        std::process::exit(1);
    }
}

// pub trait Shutdown {
//     fn shutdown(&self);
// }
