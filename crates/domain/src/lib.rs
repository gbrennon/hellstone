pub mod entities;
pub mod value_objects;
pub mod repository;
mod types;
pub mod traits;
pub use traits::{CanBeAttacked, HasAttackPower, ImmuneToDamage};
mod shared_types;