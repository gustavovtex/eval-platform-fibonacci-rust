use std::fmt;

/// The error every numkit function returns for an input it refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumError {
    OutOfRange(String),
}

impl fmt::Display for NumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumError::OutOfRange(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for NumError {}
