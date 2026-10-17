// This file defines all domain-level traits used throughout the application.

// Re-export all individual trait modules
pub mod identifiable;
pub mod displayable;

pub use identifiable::Identifiable;
mod can_be_attacked;
mod immune_to_damage;
mod has_attack_power;

pub use can_be_attacked::CanBeAttacked;
pub use immune_to_damage::ImmuneToDamage;
pub use has_attack_power::HasAttackPower;
pub use displayable::Displayable;