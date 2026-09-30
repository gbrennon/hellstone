use std::fmt;

/// Failure raised by a storage mechanism backing an outbound repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    Unavailable(String),
    Corrupted(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable(detail) => write!(formatter, "storage unavailable: {detail}"),
            Self::Corrupted(detail) => write!(formatter, "stored state corrupted: {detail}"),
        }
    }
}
