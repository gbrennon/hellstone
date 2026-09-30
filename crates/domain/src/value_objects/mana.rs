// This file was moved from src/domain/value_objects/mana.rs
// It is now part of the workspace crate.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mana(u8);

impl Mana {
    pub const BASE_MAX_MANA: u8 = 10;
    pub const STARTING_MANA: u8 = 1;

    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn current(&self) -> u8 {
        self.0
    }

    pub fn can_spend(&self, amount: u8) -> bool {
        self.0 >= amount
    }

    pub fn spend(&mut self, amount: u8) -> Result<(), ManaError> {
        if !self.can_spend(amount) {
            return Err(ManaError::InsufficientMana { available: self.0, requested: amount });
        }
        self.0 -= amount;
        Ok(())
    }

    pub fn replenish(&mut self) {
        self.0 = Self::BASE_MAX_MANA;
    }

    pub fn fill(&mut self) {
        self.0 = Self::BASE_MAX_MANA;
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn add(&mut self, amount: u8) {
        self.0 = self.0.saturating_add(amount);
    }
}

impl Default for Mana {
    fn default() -> Self {
        Self(Self::STARTING_MANA)
    }
}

impl fmt::Display for Mana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManaError {
    InsufficientMana { available: u8, requested: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mana_new() {
        let mana = Mana::new(5);
        assert_eq!(mana.current(), 5);
    }

    #[test]
    fn test_mana_can_spend() {
        let mana = Mana::new(3);
        assert!(mana.can_spend(2));
        assert!(!mana.can_spend(4));
    }

    #[test]
    fn test_mana_spend_success() {
        let mut mana = Mana::new(5);
        mana.spend(2).unwrap();
        assert_eq!(mana.current(), 3);
    }

    #[test]
    fn test_mana_spend_insufficient() {
        let mut mana = Mana::new(2);
        let result = mana.spend(5);
        assert!(result.is_err());
    }

    #[test]
    fn test_mana_replenish() {
        let mut mana = Mana::new(1);
        mana.replenish();
        assert_eq!(mana.current(), 10);
    }

    #[test]
    fn test_mana_fill() {
        let mut mana = Mana::new(1);
        mana.fill();
        assert_eq!(mana.current(), 10);
    }

    #[test]
    fn test_mana_set() {
        let mut mana = Mana::new(1);
        mana.set(7);
        assert_eq!(mana.current(), 7);
    }

    #[test]
    fn test_mana_add() {
        let mut mana = Mana::new(3);
        mana.add(5);
        assert_eq!(mana.current(), 8);
    }

    #[test]
    fn test_mana_display() {
        let mana = Mana::new(5);
        assert_eq!(format!("{}", mana), "5");
    }
}