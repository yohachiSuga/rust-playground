use std::{fmt::Display, result};

use thiserror::{private::DisplayAsDisplay, Error};
pub type Result<T> = result::Result<T, PlaygroundError>;

#[derive(Debug, Error)]
pub enum PlaygroundError {
    #[error("playground error: {0}")]
    Custom(ErrorKind),
    #[error("std lib error {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug)]
pub enum ErrorKind {
    UnknownError = 0x01,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnknownError => "unknown error",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error_code {}", &self.as_str())
    }
}
