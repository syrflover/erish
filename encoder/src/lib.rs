pub mod config;
mod error;
pub mod ffmpeg;
pub mod ffprobe;
pub mod time;

pub use error::Error;

pub type Result<T> = std::result::Result<T, error::Error>;
