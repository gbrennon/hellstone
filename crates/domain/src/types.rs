// This file defines common type aliases used throughout the domain layer.

use crate::entities::card::Card;
use crate::value_objects::{CardId, CardRarity, CardType, Health, Mana, PlayerId};

/// A collection of cards representing a player's deck.
pub type Deck = Vec<Card>;

/// A collection of cards representing a player's hand.
pub type Hand = Vec<Card>;

/// A reference to a card in the game.
pub type CardRef = CardId;

/// A reference to a player in the game.
pub type PlayerRef = PlayerId;