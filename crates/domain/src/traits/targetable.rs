/// Marks a game entity that can be selected as a target by card effects.
///
/// This capability is intentionally behavior-free; systems can use it to
/// distinguish valid targets from entities that cannot be targeted.
pub trait Targetable {}
