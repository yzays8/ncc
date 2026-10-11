use thiserror::Error;

#[derive(Error, Debug)]
pub enum NccError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Lexing(String),
    #[error("{0}")]
    Parsing(String),
    #[error("{0}")]
    CodeGeneration(String),
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, NccError>;
