mod u8_array;

pub use u8_array::*;

pub type TypeExtError = Box<dyn std::error::Error + Send + Sync>;
