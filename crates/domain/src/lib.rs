pub mod entities;
pub mod repository;
pub mod traits;
pub mod types;
pub mod value_objects;

pub use traits::{
    CanBeAttacked, Displayable, HasAttackPower, Identifiable, ImmuneToDamage, Targetable,
};
