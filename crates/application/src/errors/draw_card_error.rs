use crate::ports::outbound::RepositoryError;
use domain::entities::PlayerError;
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

impl From<PlayerError> for DrawCardError {
    fn from(error: PlayerError) -> Self {
        match error {
            PlayerError::HandFull => Self::HandFull,
            PlayerError::DeckEmpty => Self::DeckEmpty,
            PlayerError::CardNotInDeck(_) => Self::DeckEmpty,
            PlayerError::CardNotInHand(_) => Self::HandFull,
            PlayerError::DeckFull => Self::DeckEmpty,
            PlayerError::PlayerDead => Self::PlayerDead,
            PlayerError::InsufficientMana { .. } => Self::DeckEmpty,
            PlayerError::Health(_) => Self::PlayerDead,
        }
    }
}
