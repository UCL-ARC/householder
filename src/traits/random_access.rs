//! Traits for random access of matrices

use crate::types::{IndexType, Scalar};
use crate::traits::Dimensions;

/// Random access without bounds check for matrices.
pub trait UnsafeRandomAccess {
    type Item: Scalar;

    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item;
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item;
}

/// Get mutable access to element without bounds check.
pub trait UnsafeRandomAccessMut {
    type Item: Scalar;

    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item;
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item;
}

/// Bounds checked random access for matrices.
pub trait RandomAccess: UnsafeRandomAccess + Dimensions {
    /// Get the element at position (row, col) of the matrix.
    fn get(&self, row: usize, col: usize) -> Self::Item;

    /// Get element from matrix linearized as 1d array (result depends on memory layout).
    fn get1d(&self, elem: usize) -> Self::Item;
}

/// Bounds checked mutable random access for matrices.
pub trait RandomAccessMut: UnsafeRandomAccessMut + Dimensions {
    /// Get mutable reference to element at position (row, col) of the matrix.
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Item;
    /// Get mutable reference from matrix linearized as 1d array (result depends on memory layout).
    fn get1d_mut(&mut self, elem: usize) -> &mut Self::Item;
}


#[inline]
fn assert_dimension(row: IndexType, col: IndexType, dim: (IndexType, IndexType)) {
    assert!(
        row < dim.0,
        "row {} out of bounds (dim: {}, {}",
        row,
        dim.0,
        dim.1
    );
    assert!(
        col < dim.1,
        "col {} out of bounds (dim: {}, {}",
        col,
        dim.0,
        dim.1
    );
}

#[inline]
fn assert_dimension1d(elem: IndexType, nelems: IndexType) {
    assert!(
        elem < nelems,
        "elem {} out of bounds (nelems: {})",
        elem,
        nelems
    );
}

impl<Item: Scalar, Mat: UnsafeRandomAccess<Item=Item> + Dimensions> RandomAccess for Mat {
    fn get(&self, row: IndexType, col: IndexType)  -> Self::Item {
        assert_dimension(row, col, self.dim());
        unsafe {self.get_unchecked(row, col) }
    }

    fn get1d(&self, elem: IndexType) -> Self::Item {
        assert_dimension1d(elem, self.number_of_elements());
        unsafe {self.get1d_unchecked(elem)}
    }
}

impl<Item: Scalar, Mat: UnsafeRandomAccessMut<Item=Item> + Dimensions> RandomAccessMut for Mat {
    fn get_mut(&mut self, row: IndexType, col: IndexType)  -> &mut Self::Item {
        assert_dimension(row, col, self.dim());
        unsafe {self.get_unchecked_mut(row, col) }
    }

    fn get1d_mut(&mut self, elem: IndexType) -> &mut Self::Item {
        assert_dimension1d(elem, self.number_of_elements());
        unsafe {self.get1d_unchecked_mut(elem)}
    }
}
