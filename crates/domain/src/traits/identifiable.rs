use crate::value_objects::PlayerId;

/// A trait for objects that have an ID.
pub trait Identifiable {
    /// Get the unique identifier for this object.
    fn id(&self) -> &PlayerId;
}
