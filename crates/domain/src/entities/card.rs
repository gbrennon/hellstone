// This file was moved from src/domain/entities/card.rs
// It is now part of the workspace crate.
use crate::traits::{CanBeAttacked, HasAttackPower};
use crate::value_objects::{Attack, CardId, CardRarity, CardType, Health};
use std::fmt;


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
        name: &str,
        mana_cost: u8,
        card_type: CardType,
        rarity: CardRarity,
        base_attack: Option<Attack>,
        base_health: Option<Health>,
        description: &str,
    ) -> Result<Self, CardError> {
        if name.is_empty() {
            return Err(CardError::EmptyName);
        }

        match card_type {
            CardType::Spell => {
                if base_attack.is_some() || base_health.is_some() {
                    return Err(CardError::SpellCannotHaveAttack);
                }
            }
            _ => {}
        }

        Ok(Self {
            id,
            name: name.to_string(),
            mana_cost,
            card_type,
            rarity,
            base_attack,
            base_health,
            current_attack: None,
            current_health: None,
            description: description.to_string(),
            can_attack: true,
            is_sleeping: false,
        })
    }

    // Getters
    pub fn id(&self) -> &CardId { &self.id }
    pub fn name(&self) -> &String { &self.name }
    pub fn mana_cost(&self) -> u8 { self.mana_cost }
    pub fn card_type(&self) -> &CardType { &self.card_type }
    pub fn rarity(&self) -> &CardRarity { &self.rarity }
    pub fn base_attack(&self) -> &Option<Attack> { &self.base_attack }
    pub fn base_health(&self) -> &Option<Health> { &self.base_health }
    pub fn current_attack(&self) -> &Option<Attack> { &self.current_attack }
    pub fn current_health(&self) -> &Option<Health> { &self.current_health }
    pub fn description(&self) -> &String { &self.description }
    pub fn can_attack(&self) -> bool { self.can_attack }
    pub fn is_sleeping(&self) -> bool { self.is_sleeping }

    // Mutators
    pub fn set_current_attack(&mut self, attack: Attack) {
        self.current_attack = Some(attack);
    }

    pub fn set_current_health(&mut self, health: Health) {
        self.current_health = Some(health);
    }

    pub fn take_damage(&mut self, damage: u32) -> Result<u32, CardError> {
        let mut remaining_damage = damage;
        while remaining_damage > 0 {
            match self.current_health.as_mut() {
                Some(h) => {
                    let new_value = h.current().saturating_sub(remaining_damage);
                    h.set(new_value).map_err(CardError::Health)?;
                    remaining_damage = 0;
                }
                None => break,
            }
        }
        Ok(damage)
    }

    pub fn wake_up(&mut self) {
        if !self.is_sleeping {
            return;
        }
        self.is_sleeping = false;
        self.can_attack = true;
    }

    pub fn exhaust(&mut self) {
        if !self.can_attack {
            return;
        }
        self.can_attack = false;
    }

    pub fn is_dead(&self) -> bool {
        matches!(self.current_health, Some(h) if h.current() == 0)
    }
}

impl CanBeAttacked for Card {}

impl HasAttackPower for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Card {}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} | {} | {})",
               self.name,
               self.card_type,
               self.rarity,
               self.mana_cost
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardError {
    EmptyName,
    Health(crate::value_objects::HealthError),
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
            "Minion",
            3,
            CardType::Minion,
            CardRarity::Common,
            Some(Attack::new(2).unwrap()),
            Some(Health::new(4).unwrap()),
            "A test minion"
        ).unwrap()
    }

    #[test]
    fn test_card_equality_by_id() {
        let card1 = create_minion();
        let card2 = create_minion();
        assert_eq!(card1, card2);
    }

    #[test]
    fn test_card_inequality_by_different_id() {
        let mut card1 = create_minion();
        card1.id = CardId::new(2);
        let card2 = create_minion();
        assert_ne!(card1, card2);
    }

    #[test]
    fn test_card_take_damage() {
        let mut card = create_minion();
        let damage_taken = card.take_damage(5).unwrap();
        assert_eq!(damage_taken, 5);
        assert_eq!(card.current_health().as_ref().unwrap().current(), 0);
    }

    #[test]
    fn test_card_wake_up() {
        let mut card = create_minion();
        card.is_sleeping = true;
        card.wake_up();
        assert!(!card.is_sleeping);
        assert!(card.can_attack);
    }

    #[test]
    fn test_card_exhaust() {
        let mut card = create_minion();
        card.exhaust();
        assert!(!card.can_attack);
    }

    #[test]
    fn test_card_is_dead() {
        let mut card = create_minion();
        card.set_current_health(Health::new(0).unwrap());
        assert!(card.is_dead());
    }
}