use crate::value_objects::{Attack, CardId, CardRarity, CardType, Health};

/// Construction specification for a card.
///
/// Carries every attribute a card is printed with, so callers describe a card
/// through intention-revealing steps instead of positional arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardBlueprint {
    id: CardId,
    name: String,
    mana_cost: u8,
    card_type: CardType,
    rarity: CardRarity,
    base_attack: Option<Attack>,
    base_health: Option<Health>,
    description: String,
}

impl CardBlueprint {
    /// Starts a blueprint with the attributes every card carries.
    ///
    /// Combat stats and description default to empty and are added through
    /// `with_combat_stats` and `described_as`.
    pub fn new(
        id: CardId,
        name: &str,
        mana_cost: u8,
        card_type: CardType,
        rarity: CardRarity,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            mana_cost,
            card_type,
            rarity,
            base_attack: None,
            base_health: None,
            description: String::new(),
        }
    }

    /// Returns a blueprint carrying the given printed combat stats.
    pub fn with_combat_stats(mut self, attack: Attack, health: Health) -> Self {
        self.base_attack = Some(attack);
        self.base_health = Some(health);
        self
    }

    /// Returns a blueprint carrying the given rules text.
    pub fn described_as(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn id(&self) -> &CardId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mana_cost(&self) -> u8 {
        self.mana_cost
    }

    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }

    pub fn rarity(&self) -> &CardRarity {
        &self.rarity
    }

    pub fn base_attack(&self) -> Option<Attack> {
        self.base_attack
    }

    pub fn base_health(&self) -> Option<Health> {
        self.base_health
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
