use crate::ports::outbound::RepositoryError;
use std::fmt;

/// Reason a card could not be played from hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayCardError {
    PlayerNotFound(u64),
    CardNotInHand(u64),
    InsufficientMana { available: u8, requested: u8 },
    PlayerDead,
    Repository(RepositoryError),
}

impl fmt::Display for PlayCardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlayerNotFound(id) => write!(formatter, "player {id} was not found"),
            Self::CardNotInHand(id) => write!(formatter, "card {id} is not in hand"),
            Self::InsufficientMana {
                available,
                requested,
            } => write!(formatter, "needs {requested} mana, only {available} left"),
            Self::PlayerDead => write!(formatter, "a dead player cannot play a card"),
            Self::Repository(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<RepositoryError> for PlayCardError {
    fn from(error: RepositoryError) -> Self {
        Self::Repository(error)
    }
}
