// This file was created as part of the domain crate.
// It contains repository interfaces for domain entities.

use crate::entities::{Card, Player};
use crate::value_objects::{CardId, PlayerId};
use std::fmt;

/// Repository interface for Player entity.
pub trait PlayerRepository {
    /// Save a player to persistent storage.
    fn save(&self, player: &Player) -> Result<(), Box<dyn std::error::Error>>;

    /// Load a player by ID.
    fn load(&self, id: &PlayerId) -> Result<Player, Box<dyn std::error::Error>>;
}

/// Repository interface for Card entity.
pub trait CardRepository {
    /// Save a card to persistent storage.
    fn save(&self, card: &Card) -> Result<(), Box<dyn std::error::Error>>;

    /// Load a card by ID.
    fn load(&self, id: &CardId) -> Result<Card, Box<dyn std::error::Error>>;
}

/// A composite repository that can handle multiple types.
pub struct CompositeRepository {
    pub players: Box<dyn PlayerRepository>,
    pub cards: Box<dyn CardRepository>,
}

impl CompositeRepository {
    pub fn new(players: Box<dyn PlayerRepository>, cards: Box<dyn CardRepository>) -> Self {
        Self { players, cards }
    }
}

impl fmt::Debug for CompositeRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CompositeRepository {{ players: ..., cards: ... }}")
    }
}