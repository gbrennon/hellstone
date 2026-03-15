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
        if self.0 >= amount {
            self.0 -= amount;
            Ok(())
        } else {
            Err(ManaError::InsufficientMana {
                available: self.0,
                requested: amount,
            })
        }
    }

    pub fn replenish(&mut self) {
        if self.0 < Self::BASE_MAX_MANA {
            self.0 += 1;
        }
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
    fn test_mana_new_valid() {
        let mana = Mana::new(5);
        assert_eq!(mana.current(), 5);
    }

    #[test]
    fn test_mana_new_above_base_max() {
        let mana = Mana::new(15);
        assert_eq!(mana.current(), 15);
    }

    #[test]
    fn test_mana_spend_success() {
        let mut mana = Mana::new(5);
        assert!(mana.spend(3).is_ok());
        assert_eq!(mana.current(), 2);
    }

    #[test]
    fn test_mana_spend_insufficient() {
        let mut mana = Mana::new(3);
        let result = mana.spend(5);
        assert!(result.is_err());
    }

    #[test]
    fn test_mana_can_spend() {
        let mana = Mana::new(5);
        assert!(mana.can_spend(3));
        assert!(!mana.can_spend(6));
    }

    #[test]
    fn test_mana_replenish() {
        let mut mana = Mana::new(3);
        mana.replenish();
        assert_eq!(mana.current(), 4);
    }

    #[test]
    fn test_mana_replenish_at_base_max() {
        let mut mana = Mana::new(10);
        mana.replenish();
        assert_eq!(mana.current(), 10);
    }

    #[test]
    fn test_mana_replenish_above_base_max() {
        let mut mana = Mana::new(15);
        mana.replenish();
        assert_eq!(mana.current(), 15);
    }

    #[test]
    fn test_mana_fill() {
        let mut mana = Mana::new(3);
        mana.fill();
        assert_eq!(mana.current(), 10);
    }

    #[test]
    fn test_mana_add() {
        let mut mana = Mana::new(5);
        mana.add(3);
        assert_eq!(mana.current(), 8);
    }

    #[test]
    fn test_mana_add_saturates() {
        let mut mana = Mana::new(10);
        mana.add(100);
        assert_eq!(mana.current(), 110);
    }

    #[test]
    fn test_mana_default() {
        let mana = Mana::default();
        assert_eq!(mana.current(), 1);
    }

    #[test]
    fn test_mana_display() {
        let mana = Mana::new(7);
        assert_eq!(format!("{}", mana), "7");
    }
}
