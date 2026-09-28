// This file defines the Displayable trait used throughout the domain layer.

use std::fmt::Formatter;

/// A trait for objects that can be displayed as text.
pub trait Displayable {
    /// Format the object as a string.
    fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}