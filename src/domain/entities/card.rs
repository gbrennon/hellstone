use std::fmt;

use crate::domain::value_objects::{Attack, CardId, CardRarity, CardType, Health};

#[derive(Debug, Clone)]
pub struct Card {
    id: CardId,
    name: String,
    mana_cost: u8,
    card_type: CardType,
    rarity: CardRarity,
    base_attack: Option<Attack>,
    base_health: Option<Health>,
    current_attack: Option<Attack>,
    current_health: Option<Health>,
    description: String,
    can_attack: bool,
    is_sleeping: bool,
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

        let current_attack = attack;
        let current_health = health;

        Ok(Self {
            id,
            name,
            mana_cost,
            card_type,
            rarity,
            base_attack: attack,
            base_health: health,
            current_attack,
            current_health,
            description,
            can_attack: false,
            is_sleeping: matches!(card_type, CardType::Minion),
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
        self.current_attack
    }

    pub fn health(&self) -> Option<Health> {
        self.current_health
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_minion(&self) -> bool {
        self.card_type == CardType::Minion
    }

    pub fn can_attack(&self) -> bool {
        self.can_attack
            && !self.is_sleeping
            && self.current_attack.map(|a| a.can_attack()).unwrap_or(false)
    }

    pub fn is_legendary(&self) -> bool {
        self.rarity == CardRarity::Legendary
    }

    pub fn take_damage(&mut self, damage: u32) -> u32 {
        if let Some(ref mut health) = self.current_health {
            if health.current() == 0 {
                return 0;
            }
            let actual = health.take_damage(damage).unwrap_or(0);
            return actual;
        }
        0
    }

    pub fn modify_attack(&mut self, delta: i16) {
        if let Some(ref mut attack) = self.current_attack {
            let _ = attack.modify(delta);
        }
    }

    pub fn heal(&mut self, amount: u32) {
        if let Some(ref mut health) = self.current_health {
            let _ = health.heal(amount);
        }
    }

    pub fn wake_up(&mut self) {
        self.is_sleeping = false;
        self.can_attack = true;
    }

    pub fn exhaust(&mut self) {
        self.can_attack = false;
    }

    pub fn destroy(&mut self) {
        if let Some(ref mut health) = self.current_health {
            let _ = health.take_damage(u32::MAX);
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current_health.map(|h| h.is_dead()).unwrap_or(false)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Card {}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} mana)", self.name, self.mana_cost)?;
        if let Some(attack) = self.current_attack {
            write!(f, " [{}]", attack)?;
        }
        if let Some(health) = self.current_health {
            write!(f, "/{}", health)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardError {
    EmptyName,
    SpellCannotHaveAttack,
    SpellCannotHaveHealth,
    MissingAttackForCardType(CardType),
    MissingHealthForCardType(CardType),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{CardId, Health};

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

    #[test]
    fn test_card_equality_by_id() {
        let card1 = create_minion();
        let card2 = Card::new(
            CardId::new(1),
            "Different Name".to_string(),
            5,
            CardType::Minion,
            CardRarity::Legendary,
            Some(Attack::new(10).unwrap()),
            Some(Health::new(10).unwrap()),
            "Different".to_string(),
        )
        .unwrap();

        assert_eq!(card1, card2);
    }

    #[test]
    fn test_card_inequality_by_different_id() {
        let card1 = create_minion();
        let card2 = Card::new(
            CardId::new(2),
            "Test Minion".to_string(),
            3,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(3).unwrap()),
            Some(Health::new(4).unwrap()),
            "A test minion".to_string(),
        )
        .unwrap();

        assert_ne!(card1, card2);
    }

    #[test]
    fn test_card_take_damage() {
        let mut card = create_minion();
        let damage = card.take_damage(2);
        assert_eq!(damage, 2);
        assert_eq!(card.health().unwrap().current(), 2);
    }

    #[test]
    fn test_card_wake_up() {
        let mut card = create_minion();
        assert!(!card.can_attack());
        card.wake_up();
        assert!(card.can_attack());
    }

    #[test]
    fn test_card_exhaust() {
        let mut card = create_minion();
        card.wake_up();
        assert!(card.can_attack());
        card.exhaust();
        assert!(!card.can_attack());
    }

    #[test]
    fn test_card_is_dead() {
        let mut card = create_minion();
        assert!(!card.is_dead());
        card.take_damage(4);
        assert!(card.is_dead());
    }
}
