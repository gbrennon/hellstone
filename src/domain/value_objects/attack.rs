use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Attack(u8);

impl Attack {
    pub const MIN_ATTACK: u8 = 0;
    pub const MAX_ATTACK: u8 = 20;

    pub fn new(value: u8) -> Option<Self> {
        if value <= Self::MAX_ATTACK {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn current(&self) -> u8 {
        self.0
    }

    pub fn can_attack(&self) -> bool {
        self.0 > 0
    }

    pub fn set(&mut self, value: u8) -> Result<(), AttackError> {
        if value > Self::MAX_ATTACK {
            Err(AttackError::AboveMax {
                max: Self::MAX_ATTACK,
                requested: value,
            })
        } else {
            self.0 = value;
            Ok(())
        }
    }

    pub fn modify(&mut self, delta: i16) -> Result<(), AttackError> {
        let new_value = (self.0 as i16 + delta).clamp(0, Self::MAX_ATTACK as i16) as u8;
        self.0 = new_value;
        Ok(())
    }
}

impl Default for Attack {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttackError {
    AboveMax { max: u8, requested: u8 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_new_valid() {
        let attack = Attack::new(5);
        assert!(attack.is_some());
        assert_eq!(attack.unwrap().current(), 5);
    }

    #[test]
    fn test_attack_new_above_max() {
        let attack = Attack::new(21);
        assert!(attack.is_none());
    }

    #[test]
    fn test_attack_default() {
        let attack = Attack::default();
        assert_eq!(attack.current(), 0);
    }

    #[test]
    fn test_attack_can_attack() {
        let attack = Attack::new(1).unwrap();
        assert!(attack.can_attack());

        let attack = Attack::new(0).unwrap();
        assert!(!attack.can_attack());
    }

    #[test]
    fn test_attack_set() {
        let mut attack = Attack::new(3).unwrap();
        attack.set(7).unwrap();
        assert_eq!(attack.current(), 7);
    }

    #[test]
    fn test_attack_set_above_max() {
        let mut attack = Attack::new(3).unwrap();
        let result = attack.set(25);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AttackError::AboveMax {
                max: 20,
                requested: 25
            }
        );
    }

    #[test]
    fn test_attack_modify_increase() {
        let mut attack = Attack::new(3).unwrap();
        attack.modify(2).unwrap();
        assert_eq!(attack.current(), 5);
    }

    #[test]
    fn test_attack_modify_decrease() {
        let mut attack = Attack::new(5).unwrap();
        attack.modify(-3).unwrap();
        assert_eq!(attack.current(), 2);
    }

    #[test]
    fn test_attack_modify_below_zero() {
        let mut attack = Attack::new(2).unwrap();
        attack.modify(-5).unwrap();
        assert_eq!(attack.current(), 0);
    }

    #[test]
    fn test_attack_modify_above_max() {
        let mut attack = Attack::new(18).unwrap();
        attack.modify(5).unwrap();
        assert_eq!(attack.current(), 20);
    }

    #[test]
    fn test_attack_display() {
        let attack = Attack::new(7).unwrap();
        assert_eq!(format!("{}", attack), "7");
    }

    #[test]
    fn test_attack_ord() {
        let a1 = Attack::new(3).unwrap();
        let a2 = Attack::new(5).unwrap();
        assert!(a1 < a2);
    }
}
