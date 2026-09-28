// This file was moved from src/domain/value_objects/health.rs
// It is now part of the workspace crate.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Health(u32);

impl Health {
    pub const MAX_HEALTH: u32 = 30;
    pub const STARTING_HEALTH: u32 = 30;

    pub fn new(value: u32) -> Option<Self> {
        if value > Self::MAX_HEALTH {
            return None;
        }
        Some(Self(value))
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }

    pub fn take_damage(&mut self, damage: u32) -> Result<u32, HealthError> {
        let mut remaining_damage = damage;
        while remaining_damage > 0 {
            match self.0.checked_sub(remaining_damage) {
                Some(new_value) => {
                    self.0 = new_value;
                    remaining_damage = 0;
                }
                None => break,
            }
        }
        Ok(damage)
    }

    pub fn heal(&mut self, amount: u32) -> Result<u32, HealthError> {
        let new_value = self.0.saturating_add(amount);
        if new_value > Self::MAX_HEALTH {
            return Err(HealthError::AboveMaxHealth { max: Self::MAX_HEALTH, requested: new_value });
        }
        self.0 = new_value;
        Ok(amount)
    }

    pub fn set(&mut self, value: u32) -> Result<(), HealthError> {
        if value == 0 {
            return Err(HealthError::CannotSetToZero);
        }
        if value > Self::MAX_HEALTH {
            return Err(HealthError::AboveMaxHealth { max: Self::MAX_HEALTH, requested: value });
        }
        self.0 = value;
        Ok(())
    }
}

impl Default for Health {
    fn default() -> Self {
        Self(Self::STARTING_HEALTH)
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthError {
    AlreadyDead,
    CannotSetToZero,
    AboveMaxHealth { max: u32, requested: u32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new_valid() {
        let health = Health::new(15).unwrap();
        assert_eq!(health.current(), 15);
    }

    #[test]
    fn test_health_new_above_max() {
        let result = Health::new(31);
        assert!(result.is_none());
    }

    #[test]
    fn test_health_default() {
        let health = Health::default();
        assert_eq!(health.current(), 30);
    }

    #[test]
    fn test_health_is_dead() {
        let mut health = Health::new(0).unwrap();
        assert!(health.is_dead());
        health.set(1).unwrap();
        assert!(!health.is_dead());
    }

    #[test]
    fn test_health_take_damage() {
        let mut health = Health::new(10).unwrap();
        let damage_taken = health.take_damage(7).unwrap();
        assert_eq!(damage_taken, 7);
        assert_eq!(health.current(), 3);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(5).unwrap();
        let healed_amount = health.heal(8).unwrap();
        assert_eq!(healed_amount, 8);
        assert_eq!(health.current(), 13);
    }

    #[test]
    fn test_health_heal_above_max() {
        let mut health = Health::new(29).unwrap();
        let result = health.heal(2);
        assert!(result.is_err());
    }

    #[test]
    fn test_health_set_to_zero() {
        let mut health = Health::new(10).unwrap();
        let result = health.set(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_health_set_above_max() {
        let mut health = Health::new(10).unwrap();
        let result = health.set(31);
        assert!(result.is_err());
    }

    #[test]
    fn test_health_display() {
        let health = Health::new(5).unwrap();
        assert_eq!(format!("{}", health), "5");
    }
}