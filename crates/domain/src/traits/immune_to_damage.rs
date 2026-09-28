/// Marks an entity that cannot receive damage.
///
/// This marker capability is intentionally behavior-free: systems can use it
/// to distinguish damage-immune entities from entities that can take damage.
pub trait ImmuneToDamage {}
