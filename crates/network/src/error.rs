use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Custom error: {0}")]
    Custom(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NetworkError>;