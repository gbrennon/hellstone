use crate::ports::outbound::RepositoryError;
use std::fmt;

/// Reason a player could not be created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreatePlayerError {
    EmptyName,
    PlayerAlreadyExists(u64),
    DeckSizeOutOfRange { maximum: usize, requested: usize },
    HandSizeOutOfRange { maximum: usize, requested: usize },
    Repository(RepositoryError),
}

impl fmt::Display for CreatePlayerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(formatter, "player name cannot be empty"),
            Self::PlayerAlreadyExists(id) => write!(formatter, "player {id} already exists"),
            Self::DeckSizeOutOfRange { maximum, requested } => {
                write!(
                    formatter,
                    "deck of {requested} exceeds the limit of {maximum}"
                )
            }
            Self::HandSizeOutOfRange { maximum, requested } => {
                write!(
                    formatter,
                    "hand of {requested} exceeds the limit of {maximum}"
                )
            }
            Self::Repository(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<RepositoryError> for CreatePlayerError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}
