mod u128_ext;
mod u8_array;

pub use u128_ext::*;
pub use u8_array::*;

pub type TypeExtError = Box<dyn std::error::Error + Send + Sync>;
