
// Re-export all value objects
pub use crate::value_objects::*;

// Re-export all entities
pub use crate::entities::*;

// Re-export all repositories
pub use crate::repository::*;

/// A trait for objects that have an ID.
pub trait Identifiable {
    /// Get the unique identifier for this object.
    fn id(&self) -> &PlayerId;
}

/// A trait for objects that can be displayed as text.
pub trait Displayable {
    /// Format the object as a string.
    fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

/// A trait for objects that can take damage.
pub trait Damagable {
    /// Take damage from an attack.
    fn take_damage(&mut self, damage: u32) -> Result<u32, HealthError>;
}