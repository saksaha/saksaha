use once_cell::sync::OnceCell;
use std::sync::Arc;
use logger::{terr, tinfo};

static INSTANCE: OnceCell<Process> = OnceCell::new();

pub struct Process {
    shutdownable: Arc<dyn Shutdown + Sync + Send>,
}

impl Process {
    pub fn init(shutdownable: Arc<dyn Shutdown + Sync + Send>) {
        let p = Process { shutdownable };

        match INSTANCE.set(p) {
            Ok(_) => (),
            Err(_) => {
                terr!("saksaha", "", "Cannot initialize process");

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

        process.shutdownable.shutdown();

        std::process::exit(1);
    }
}

pub trait Shutdown {
    fn shutdown(&self);
}
