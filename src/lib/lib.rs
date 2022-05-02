pub mod dirutils;
pub mod observer;

pub type KekwResult<T> = Result<T, Box<dyn std::error::Error>>;
