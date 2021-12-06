use log::{error, info};
use once_cell::sync::OnceCell;
use std::sync::Arc;

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
                error!("Cannot initialize process");

                std::process::exit(1);
            }
        }
    }

    pub fn shutdown() {
        let process = match INSTANCE.get() {
            Some(p) => p,
            None => {
                error!(
                    "Process is not initialized. Consider calling \
                    Process:init() at the launch of the program"
                );

                std::process::exit(1);
            }
        };

        info!("Calling shutdown callback");

        process.shutdownable.shutdown();

        std::process::exit(1);
    }
}

pub trait Shutdown {
    fn shutdown(&self);
}
