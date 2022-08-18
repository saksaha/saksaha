mod channel;

pub use channel::*;

pub type EnvelopeTypeError = Box<dyn std::error::Error + Send + Sync>;
