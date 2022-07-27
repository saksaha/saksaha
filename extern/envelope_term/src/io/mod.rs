pub mod handler;

use std::time::Duration;

// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,           // Launch to initialize the application
    Sleep(Duration),      // Just take a little break
    Receive(Vec<String>), // receive data from network
}

#[derive(Debug)]
pub(crate) enum InputMode {
    Normal,
    Editing,
}
