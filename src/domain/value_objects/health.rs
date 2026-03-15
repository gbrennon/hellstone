use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Health(u32);

impl Health {
    pub const MAX_HEALTH: u32 = 30;
    pub const STARTING_HEALTH: u32 = 30;

    pub fn new(value: u32) -> Option<Self> {
        if value > 0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn current(&self) -> u32 {
        self.0
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }

    pub fn take_damage(&mut self, damage: u32) -> Result<u32, HealthError> {
        if self.0 == 0 {
            return Err(HealthError::AlreadyDead);
        }

        let actual_damage = damage.min(self.0);
        self.0 -= actual_damage;
        Ok(actual_damage)
    }

    pub fn heal(&mut self, amount: u32) -> Result<u32, HealthError> {
        if self.0 == 0 {
            return Err(HealthError::AlreadyDead);
        }

        let old_health = self.0;
        self.0 = (self.0 + amount).min(Self::MAX_HEALTH);
        Ok(self.0 - old_health)
    }

    pub fn set(&mut self, value: u32) -> Result<(), HealthError> {
        if value == 0 {
            Err(HealthError::CannotSetToZero)
        } else if value > Self::MAX_HEALTH {
            Err(HealthError::AboveMaxHealth {
                max: Self::MAX_HEALTH,
                requested: value,
            })
        } else {
            self.0 = value;
            Ok(())
        }
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
        let health = Health::new(25);
        assert!(health.is_some());
        assert_eq!(health.unwrap().current(), 25);
    }

    #[test]
    fn test_health_new_invalid_zero() {
        let health = Health::new(0);
        assert!(health.is_none());
    }

    #[test]
    fn test_health_default() {
        let health = Health::default();
        assert_eq!(health.current(), 30);
    }

    #[test]
    fn test_health_is_dead() {
        let health = Health::new(0);
        assert!(health.is_none());

        let health = Health::new(1).unwrap();
        assert!(!health.is_dead());
    }

    #[test]
    fn test_health_take_damage() {
        let mut health = Health::new(10).unwrap();
        let damage = health.take_damage(4);
        assert!(damage.is_ok());
        assert_eq!(damage.unwrap(), 4);
        assert_eq!(health.current(), 6);
    }

    #[test]
    fn test_health_take_more_damage_than_health() {
        let mut health = Health::new(5).unwrap();
        let damage = health.take_damage(10);
        assert!(damage.is_ok());
        assert_eq!(damage.unwrap(), 5);
        assert_eq!(health.current(), 0);
    }

    #[test]
    fn test_health_take_damage_from_dead() {
        let mut health = Health::new(1).unwrap();
        health.take_damage(1).unwrap();
        assert!(health.is_dead());
        let result = health.take_damage(1);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HealthError::AlreadyDead);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(5).unwrap();
        let healed = health.heal(10);
        assert!(healed.is_ok());
        assert_eq!(healed.unwrap(), 10);
        assert_eq!(health.current(), 15);
    }

    #[test]
    fn test_health_heal_above_max() {
        let mut health = Health::new(25).unwrap();
        let healed = health.heal(10);
        assert!(healed.is_ok());
        assert_eq!(healed.unwrap(), 5);
        assert_eq!(health.current(), 30);
    }

    #[test]
    fn test_health_heal_dead() {
        let mut health = Health::new(1).unwrap();
        health.take_damage(1).unwrap();
        assert!(health.is_dead());
        let result = health.heal(10);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HealthError::AlreadyDead);
    }

    #[test]
    fn test_health_set() {
        let mut health = Health::new(10).unwrap();
        health.set(20).unwrap();
        assert_eq!(health.current(), 20);
    }

    #[test]
    fn test_health_set_zero_fails() {
        let mut health = Health::new(10).unwrap();
        let result = health.set(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), HealthError::CannotSetToZero);
    }

    #[test]
    fn test_health_set_above_max_fails() {
        let mut health = Health::new(10).unwrap();
        let result = health.set(31);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            HealthError::AboveMaxHealth {
                max: 30,
                requested: 31
            }
        );
    }

    #[test]
    fn test_health_display() {
        let health = Health::new(20).unwrap();
        assert_eq!(format!("{}", health), "20");
    }
}
