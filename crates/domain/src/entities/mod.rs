// This file defines common entities used throughout the domain layer.

// Re-export all individual entities
pub mod card;
pub mod player;

// Re-export all public items
pub use card::{Card, CardError};
mod creature;
pub use player::{Player, PlayerError, BASE_MAX_MANA, MAX_DECK_SIZE, MAX_HAND_SIZE, STARTING_DECK_SIZE, STARTING_HAND_SIZE};