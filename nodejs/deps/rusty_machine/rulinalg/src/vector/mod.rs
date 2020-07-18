//! The vector module.
//!
//! Currently contains all code
//! relating to the vector linear algebra struct.
use serde::{Serialize, Deserialize};

mod impl_ops;
mod impl_vec;

/// The Vector struct.
///
/// Can be instantiated with any type.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Vector<T> {
    size: usize,
    data: Vec<T>,
}