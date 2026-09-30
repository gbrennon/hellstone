use crate::ports::outbound::RepositoryError;
use std::fmt;

/// Reason a card could not be drawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawCardError {
    PlayerNotFound(u64),
    DeckEmpty,
    HandFull,
    PlayerDead,
    Repository(RepositoryError),
}

impl fmt::Display for DrawCardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlayerNotFound(id) => write!(formatter, "player {id} was not found"),
            Self::DeckEmpty => write!(formatter, "deck has no card left to draw"),
            Self::HandFull => write!(formatter, "hand cannot hold another card"),
            Self::PlayerDead => write!(formatter, "a dead player cannot draw"),
            Self::Repository(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<RepositoryError> for DrawCardError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}
