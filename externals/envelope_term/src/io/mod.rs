pub mod handler;

use std::time::Duration;

// For this dummy application we only need two IO event
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,           // Launch to initialize the application
    Sleep(Duration),      // Just take a little break
    GetChList(Vec<u8>),   // receive data from network
    GetMessages(Vec<u8>), // receive data from network
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}
