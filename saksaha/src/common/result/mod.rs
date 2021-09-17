pub mod error;

pub mod errorkind;

pub type SakResult<T> = Result<T, error::Error>;
