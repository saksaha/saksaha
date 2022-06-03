use super::System;
use log::{error, info};

impl System {
    pub(crate) fn shutdown() {
        let _system = match super::system::INSTANCE.get() {
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

        std::process::exit(1);
    }
}
