//! Basic traits for matrices
use cauchy::Scalar;

/// Bounds checked random access for matrices.
pub trait SafeRandomAccess {
    type Output: Scalar;

    /// Get the element at position (row, col) of the matrix.
    fn get(&self, row: usize, col: usize) -> Self::Output;
}

/// Bounds checked mutable random access for matrices.
pub trait SafeMutableRandomAccess {
    type Output: Scalar;

    /// Get mutable reference to element at position (row, col) of the matrix.
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output;
}


/// Random access without bounds check for matrices.
pub trait UnsafeRandomAccess {
    type Output: Scalar;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output;

}

/// Get mutable access to element without bounds check.
pub trait UnsafeMutableRandomAccess {
    type Output: Scalar;

    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output;

}

/// General trait specifying random access
pub trait RandomAccess<Item: Scalar>: SafeRandomAccess<Output=Item> + UnsafeRandomAccess<Output=Item> {}

impl<Item: Scalar, T: SafeRandomAccess<Output=Item> + UnsafeRandomAccess<Output=Item>> RandomAccess<Item> for T {}

/// Marker trait for C Layout.
pub trait CLayout {}

/// Marker trait for Fortran Layout.
pub trait FortranLayout {}

/// Any matrix type that has an associated dimension.
pub trait Dimensions {

    /// Return a tuple (row, col) specifying the dimension of the matrix.
    fn dim(&self) -> (usize, usize); 

}