//! Definition of Size Traits.
//!

// Data types to specify types of fixed size or dynamic matrices

//1 Fixed Dimension 1
pub struct Fixed1;

/// Fixed Dimension 2

pub struct Fixed2;

/// Fixed Dimension 3
pub struct Fixed3;

/// Dynamically sized dimension
pub struct Dynamic;

pub trait SizeIdentifier {
    const N: usize;
}

impl SizeIdentifier for Fixed1 {
    const N: usize = 1;

}

impl SizeIdentifier for Fixed2 {
    const N: usize = 2;
}
impl SizeIdentifier for Fixed3 {
    const N: usize= 3;
}
impl SizeIdentifier for Dynamic {
    const N: usize = 0;
}

pub trait SizeType {
    type R: SizeIdentifier;
    type C: SizeIdentifier;
}

