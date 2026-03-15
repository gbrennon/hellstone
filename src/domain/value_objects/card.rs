use std::fmt;

use super::{Attack, AttackError, CardRarity, CardType, Health};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    id: CardId,
    name: String,
    mana_cost: u8,
    card_type: CardType,
    rarity: CardRarity,
    attack: Option<Attack>,
    health: Option<Health>,
    description: String,
}

impl Card {
    pub fn new(
        id: CardId,
        name: String,
        mana_cost: u8,
        card_type: CardType,
        rarity: CardRarity,
        attack: Option<Attack>,
        health: Option<Health>,
        description: String,
    ) -> Result<Self, CardError> {
        if name.trim().is_empty() {
            return Err(CardError::EmptyName);
        }
        if mana_cost > 10 {
            return Err(CardError::ManaCostTooHigh(mana_cost));
        }

        if let (Some(_), CardType::Spell) = (&attack, card_type) {
            return Err(CardError::SpellCannotHaveAttack);
        }
        if let (Some(_), CardType::Spell) = (&health, card_type) {
            return Err(CardError::SpellCannotHaveHealth);
        }

        let requires_attack = matches!(card_type, CardType::Minion | CardType::Weapon);
        let requires_health = matches!(card_type, CardType::Minion);

        if requires_attack && attack.is_none() {
            return Err(CardError::MissingAttackForCardType(card_type));
        }
        if requires_health && health.is_none() {
            return Err(CardError::MissingHealthForCardType(card_type));
        }

        Ok(Self {
            id,
            name,
            mana_cost,
            card_type,
            rarity,
            attack,
            health,
            description,
        })
    }

    pub fn id(&self) -> CardId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mana_cost(&self) -> u8 {
        self.mana_cost
    }

    pub fn card_type(&self) -> CardType {
        self.card_type
    }

    pub fn rarity(&self) -> CardRarity {
        self.rarity
    }

    pub fn attack(&self) -> Option<Attack> {
        self.attack
    }

    pub fn health(&self) -> Option<Health> {
        self.health
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_minion(&self) -> bool {
        self.card_type == CardType::Minion
    }

    pub fn can_attack(&self) -> bool {
        self.attack.map(|a| a.can_attack()).unwrap_or(false)
    }

    pub fn is_legendary(&self) -> bool {
        self.rarity == CardRarity::Legendary
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} mana)", self.name, self.mana_cost)?;
        if let Some(attack) = self.attack {
            write!(f, " [{}]", attack)?;
        }
        if let Some(health) = self.health {
            write!(f, "/{}", health)?;
        }
        Ok(())
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardError {
    EmptyName,
    ManaCostTooHigh(u8),
    SpellCannotHaveAttack,
    SpellCannotHaveHealth,
    MissingAttackForCardType(CardType),
    MissingHealthForCardType(CardType),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_minion() -> Card {
        Card::new(
            CardId::new(1),
            "Test Minion".to_string(),
            3,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(3).unwrap()),
            Some(Health::new(4).unwrap()),
            "A test minion".to_string(),
        )
        .unwrap()
    }

    fn create_spell() -> Card {
        Card::new(
            CardId::new(2),
            "Test Spell".to_string(),
            2,
            CardType::Spell,
            CardRarity::Rare,
            None,
            None,
            "A test spell".to_string(),
        )
        .unwrap()
    }

    fn create_weapon() -> Card {
        Card::new(
            CardId::new(3),
            "Test Weapon".to_string(),
            4,
            CardType::Weapon,
            CardRarity::Epic,
            Some(Attack::new(5).unwrap()),
            Some(Health::new(2).unwrap()),
            "A test weapon".to_string(),
        )
        .unwrap()
    }

    #[test]
    fn test_card_create_minion() {
        let card = create_minion();
        assert_eq!(card.name(), "Test Minion");
        assert_eq!(card.mana_cost(), 3);
        assert_eq!(card.card_type(), CardType::Minion);
        assert_eq!(card.attack().unwrap().current(), 3);
        assert_eq!(card.health().unwrap().current(), 4);
    }

    #[test]
    fn test_card_create_spell() {
        let card = create_spell();
        assert_eq!(card.card_type(), CardType::Spell);
        assert!(card.attack().is_none());
        assert!(card.health().is_none());
    }

    #[test]
    fn test_card_create_empty_name() {
        let result = Card::new(
            CardId::new(1),
            "".to_string(),
            3,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(3).unwrap()),
            Some(Health::new(4).unwrap()),
            "desc".to_string(),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CardError::EmptyName);
    }

    #[test]
    fn test_card_mana_cost_too_high() {
        let result = Card::new(
            CardId::new(1),
            "Test".to_string(),
            11,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(3).unwrap()),
            Some(Health::new(4).unwrap()),
            "desc".to_string(),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CardError::ManaCostTooHigh(11));
    }

    #[test]
    fn test_card_spell_with_attack_fails() {
        let result = Card::new(
            CardId::new(1),
            "Test".to_string(),
            2,
            CardType::Spell,
            CardRarity::Common,
            Some(Attack::new(3).unwrap()),
            None,
            "desc".to_string(),
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CardError::SpellCannotHaveAttack);
    }

    #[test]
    fn test_card_minion_without_attack_fails() {
        let result = Card::new(
            CardId::new(1),
            "Test".to_string(),
            3,
            CardType::Minion,
            CardRarity::Common,
            None,
            Some(Health::new(4).unwrap()),
            "desc".to_string(),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CardError::MissingAttackForCardType(CardType::Minion)
        );
    }

    #[test]
    fn test_card_minion_without_health_fails() {
        let result = Card::new(
            CardId::new(1),
            "Test".to_string(),
            3,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(3).unwrap()),
            None,
            "desc".to_string(),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CardError::MissingHealthForCardType(CardType::Minion)
        );
    }

    #[test]
    fn test_card_is_minion() {
        assert!(create_minion().is_minion());
        assert!(!create_spell().is_minion());
    }

    #[test]
    fn test_card_can_attack() {
        assert!(create_minion().can_attack());
        assert!(!create_spell().can_attack());
    }

    #[test]
    fn test_card_is_legendary() {
        let legendary = Card::new(
            CardId::new(1),
            "Legendary".to_string(),
            5,
            CardType::Minion,
            CardRarity::Legendary,
            Some(Attack::new(4).unwrap()),
            Some(Health::new(4).unwrap()),
            "desc".to_string(),
        )
        .unwrap();
        assert!(legendary.is_legendary());
        assert!(!create_minion().is_legendary());
    }

    #[test]
    fn test_card_display() {
        let card = create_minion();
        assert_eq!(format!("{}", card), "Test Minion (3 mana) [3]/4");
    }

    #[test]
    fn test_card_id() {
        let card = create_minion();
        assert_eq!(card.id(), CardId::new(1));
    }

    #[test]
    fn test_card_id_display() {
        assert_eq!(format!("{}", CardId::new(42)), "CardId(42)");
    }
}
