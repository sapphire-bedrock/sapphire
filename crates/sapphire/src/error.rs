use network::error::NetworkError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    //#[error("Custom error: {0}")]
    //Custom(String),
    #[error("Network Error: {0}")]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
