use super::System;
use logger::{terr, tinfo};

impl System {
    pub(crate) fn shutdown() {
        let _system = match super::INSTANCE.get() {
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

        std::process::exit(1);
    }
}
