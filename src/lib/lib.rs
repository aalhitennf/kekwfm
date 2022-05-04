pub mod dirutils;
pub mod diskinfo;
pub mod fileutils;
pub mod history;
pub mod locations;
pub mod observer;

pub type KekwResult<T> = Result<T, Box<dyn std::error::Error>>;
