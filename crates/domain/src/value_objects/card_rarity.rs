use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardRarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl CardRarity {
    pub fn dust_value(&self) -> u32 {
        match self {
            Self::Common => 5,
            Self::Rare => 20,
            Self::Epic => 100,
            Self::Legendary => 400,
        }
    }

    pub fn crafting_cost(&self) -> u32 {
        match self {
            Self::Common => 50,
            Self::Rare => 100,
            Self::Epic => 400,
            Self::Legendary => 1600,
        }
    }
}

impl fmt::Display for CardRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Common => write!(f, "Common"),
            Self::Rare => write!(f, "Rare"),
            Self::Epic => write!(f, "Epic"),
            Self::Legendary => write!(f, "Legendary"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_rarity_common() {
        let rarity = CardRarity::Common;
        assert_eq!(rarity.dust_value(), 5);
        assert_eq!(rarity.crafting_cost(), 50);
    }

    #[test]
    fn test_card_rarity_rare() {
        let rarity = CardRarity::Rare;
        assert_eq!(rarity.dust_value(), 20);
        assert_eq!(rarity.crafting_cost(), 100);
    }

    #[test]
    fn test_card_rarity_epic() {
        let rarity = CardRarity::Epic;
        assert_eq!(rarity.dust_value(), 100);
        assert_eq!(rarity.crafting_cost(), 400);
    }

    #[test]
    fn test_card_rarity_legendary() {
        let rarity = CardRarity::Legendary;
        assert_eq!(rarity.dust_value(), 400);
        assert_eq!(rarity.crafting_cost(), 1600);
    }

    #[test]
    fn test_card_rarity_display() {
        assert_eq!(format!("{}", CardRarity::Common), "Common");
        assert_eq!(format!("{}", CardRarity::Rare), "Rare");
        assert_eq!(format!("{}", CardRarity::Epic), "Epic");
        assert_eq!(format!("{}", CardRarity::Legendary), "Legendary");
    }

    #[test]
    fn test_card_rarity_equality() {
        assert_eq!(CardRarity::Common, CardRarity::Common);
        assert_ne!(CardRarity::Common, CardRarity::Rare);
    }
}
