use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CardId(u64);

impl CardId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn current(&self) -> u64 {
        self.0
    }
}

impl fmt::Display for CardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CardId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_id_new() {
        let id = CardId::new(1);
        assert_eq!(id.current(), 1);
    }

    #[test]
    fn test_card_id_display() {
        assert_eq!(format!("{}", CardId::new(42)), "CardId(42)");
    }

    #[test]
    fn test_card_id_equality() {
        assert_eq!(CardId::new(1), CardId::new(1));
        assert_ne!(CardId::new(1), CardId::new(2));
    }
}
