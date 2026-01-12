// src/errors.rs

use std::fmt;

/// Core error type for the execution engine.
#[derive(Debug)]
pub enum Error {
    /// Generic message-based error.
    Message(String),

    /// Error for invalid input or decoding issues.
    InvalidInput(String),

    /// Error for missing state or keys.
    NotFound(String),
}

impl Error {
    pub fn msg<S: Into<String>>(msg: S) -> Self {
        Error::Message(msg.into())
    }
}

/// Convenient result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "Error: {}", msg),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
