use log::{error, info};

pub struct ShutdownMng;

impl ShutdownMng {
    pub(crate) fn shutdown(&self) {
        // TODO Shutdown behavior has to be implemented later
        info!("Calling shutdown callback");

        std::process::exit(1);
    }
}
