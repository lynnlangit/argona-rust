use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AgronaError {
    #[error("Index out of bounds: index {index}, length {length}, capacity {capacity}")]
    IndexOutOfBounds {
        index: usize,
        length: usize,
        capacity: usize,
    },

    #[error("Invalid capacity: {capacity}")]
    InvalidCapacity { capacity: usize },

    #[error("Buffer overflow: attempted to write {attempted} bytes, available {available}")]
    BufferOverflow {
        attempted: usize,
        available: usize,
    },

    #[error("ASCII number format error: {0}")]
    AsciiNumberFormat(String),

    #[error("UTF-8 encoding error: {0}")]
    Utf8Error(#[from] core::str::Utf8Error),
}

pub type Result<T> = core::result::Result<T, AgronaError>;