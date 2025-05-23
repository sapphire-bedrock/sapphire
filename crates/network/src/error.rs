use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Custom error: {0}")]
    Custom(String),

    #[error("Buffer too short: expected at least {expected}, got {actual}")]
    BufferTooShort { expected: usize, actual: usize },

    #[error("Invalid magic bytes")]
    InvalidMagicBytes,

    #[error("MOTD string too long: {0} bytes (max {1})")]
    MotdTooLong(usize, usize),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NetworkError>;
