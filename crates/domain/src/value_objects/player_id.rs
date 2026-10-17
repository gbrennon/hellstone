// This file was moved from src/domain/value_objects/player_id.rs
// It is now part of the workspace crate.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn current(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_id_new() {
        let id = PlayerId::new(1);
        assert_eq!(id.current(), 1);
    }

    #[test]
    fn test_player_id_display() {
        assert_eq!(format!("{}", PlayerId::new(42)), "PlayerId(42)");
    }

    #[test]
    fn test_player_id_equality() {
        assert_eq!(PlayerId::new(1), PlayerId::new(1));
        assert_ne!(PlayerId::new(1), PlayerId::new(2));
    }
}