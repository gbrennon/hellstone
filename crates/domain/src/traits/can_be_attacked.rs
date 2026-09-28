/// Marks an entity as a valid target for attacks.
///
/// This capability is intentionally behavior-free: combat systems can use it
/// to distinguish attackable entities from entities that cannot be targeted.
pub trait CanBeAttacked {}
