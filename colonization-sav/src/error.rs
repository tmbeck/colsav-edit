use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("invalid magic: expected COLONIZE\\0")]
    InvalidMagic,

    #[error("unexpected end of data at offset {offset}: needed {needed} bytes, got {available}")]
    UnexpectedEof {
        offset: usize,
        needed: usize,
        available: usize,
    },

    #[error("invalid section size: {section} expected {expected} bytes, got {actual}")]
    InvalidSize {
        section: &'static str,
        expected: usize,
        actual: usize,
    },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SaveError>;
