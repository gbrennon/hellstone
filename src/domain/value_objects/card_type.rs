use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    Minion,
    Spell,
    Weapon,
}

impl CardType {
    pub fn is_playable_on_board(&self) -> bool {
        matches!(self, Self::Minion | Self::Weapon)
    }

    pub fn triggers_battlecry(&self) -> bool {
        matches!(self, Self::Minion)
    }

    pub fn triggers_deathrattle(&self) -> bool {
        matches!(self, Self::Minion)
    }
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minion => write!(f, "Minion"),
            Self::Spell => write!(f, "Spell"),
            Self::Weapon => write!(f, "Weapon"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_type_minion_playable() {
        let card_type = CardType::Minion;
        assert!(card_type.is_playable_on_board());
        assert!(card_type.triggers_battlecry());
        assert!(card_type.triggers_deathrattle());
    }

    #[test]
    fn test_card_type_spell_not_playable() {
        let card_type = CardType::Spell;
        assert!(!card_type.is_playable_on_board());
        assert!(!card_type.triggers_battlecry());
        assert!(!card_type.triggers_deathrattle());
    }

    #[test]
    fn test_card_type_weapon_playable() {
        let card_type = CardType::Weapon;
        assert!(card_type.is_playable_on_board());
        assert!(!card_type.triggers_battlecry());
        assert!(!card_type.triggers_deathrattle());
    }

    #[test]
    fn test_card_type_display() {
        assert_eq!(format!("{}", CardType::Minion), "Minion");
        assert_eq!(format!("{}", CardType::Spell), "Spell");
        assert_eq!(format!("{}", CardType::Weapon), "Weapon");
    }

    #[test]
    fn test_card_type_equality() {
        assert_eq!(CardType::Minion, CardType::Minion);
        assert_ne!(CardType::Minion, CardType::Spell);
    }
}
