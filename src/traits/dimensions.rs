//! Trait to define the dimension property of a matrix.

use crate::types::IndexType;

/// Any matrix type that has an associated dimension.
pub trait Dimensions {
    /// Return a tuple (row, col) specifying the dimension of the matrix.
    fn dim(&self) -> (IndexType, IndexType);

    /// Return the number of elements.
    fn number_of_elements(&self) -> IndexType;
}
