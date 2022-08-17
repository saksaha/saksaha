mod frame;
pub mod frame_io;
mod parse;

pub use frame::Frame;
pub use parse::*;

pub type FrameError = Box<dyn std::error::Error + Send + Sync>;
