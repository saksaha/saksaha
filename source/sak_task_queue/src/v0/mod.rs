mod task_queue;

pub use task_queue::*;

pub type TaskQueueError = Box<dyn std::error::Error + Send + Sync>;
