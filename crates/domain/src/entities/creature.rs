use crate::traits::{CanBeAttacked, HasAttackPower, Targetable};
use crate::value_objects::{Attack, CardId, Health, Mana};

/// A playable unit that occupies the board, can be attacked, and deals damage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Creature {
    id: CardId,
    name: String,
    health: Health,
    attack: Attack,
    mana_cost: Mana,
}

impl Creature {
    /// Builds a creature from its identity, vitality, offense, and cost.
    pub fn new(id: CardId, name: String, health: Health, attack: Attack, mana_cost: Mana) -> Self {
        Self {
            id,
            name,
            health,
            attack,
            mana_cost,
        }
    }

    /// Returns the identifier distinguishing this creature from every other.
    pub fn id(&self) -> &CardId {
        &self.id
    }

    /// Returns the display name of this creature.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the vitality currently remaining for this creature.
    pub fn health(&self) -> &Health {
        &self.health
    }

    /// Returns the offensive power this creature applies when attacking.
    pub fn attack(&self) -> &Attack {
        &self.attack
    }

    /// Returns the mana required to summon this creature.
    pub fn mana_cost(&self) -> &Mana {
        &self.mana_cost
    }
}

impl CanBeAttacked for Creature {}

impl HasAttackPower for Creature {}

impl Targetable for Creature {}
