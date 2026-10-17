// A playable unit in the game that can be attacked and deal damage.

use crate::value_objects::{CardId, CardType, Health, Mana};// Add other necessary types as needed

/// Represents a creature card in the game.
#[derive(Debug, Clone)]
pub struct Creature {
    /// Unique identifier for the creature.
    pub id: CardId,

    /// The name of the creature.
    pub name: String,

    /// The current health points of the creature.
    pub health: Health,

    /// The base attack power of the creature.
    pub attack_power: u32,

    /// The cost to play this creature (mana).
    pub mana_cost: Mana,

    /// Whether the creature is immune to damage.
    pub immune_to_damage: bool,
}

impl Creature {
    /// Creates a new creature with given properties.
    pub fn new(id: CardId, name: String, health: Health, attack_power: u32, mana_cost: Mana) -> Self {
        Self {
            id,
            name,
            health,
            attack_power,
            mana_cost,
            immune_to_damage: false,
        }
    }

    /// Returns the current health of the creature.
    pub fn get_health(&self) -> &Health {
        &self.health
    }

    /// Returns the attack power of the creature.
    pub fn get_attack_power(&self) -> u32 {
        self.attack_power
    }
}

// Marker traits for gameplay capabilities
pub trait CanBeAttacked {}
pub trait ImmuneToDamage {}
pub trait HasAttackPower {}

// Implement marker traits
impl CanBeAttacked for Creature {}
impl ImmuneToDamage for Creature {}
impl HasAttackPower for Creature {}