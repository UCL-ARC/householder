//! Definition of Size Traits.
//!

pub enum MatrixSizeType {
    ONE,
    TWO,
    THREE,
    DYNAMIC,
}

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
    const IDENT: MatrixSizeType;
}

impl SizeIdentifier for Fixed1 {
    const IDENT: MatrixSizeType = MatrixSizeType::ONE;
}
impl SizeIdentifier for Fixed2 {
    const IDENT: MatrixSizeType = MatrixSizeType::TWO;
}
impl SizeIdentifier for Fixed3 {
    const IDENT: MatrixSizeType = MatrixSizeType::THREE;
}
impl SizeIdentifier for Dynamic {
    const IDENT: MatrixSizeType = MatrixSizeType::DYNAMIC;
}

pub trait SizeType {
    type R: SizeIdentifier;
    type C: SizeIdentifier;
}

pub trait Size<R: SizeIdentifier, C: SizeIdentifier>: SizeType<R = R, C = C> {
    // Return the size type of the object
    fn size_type(&self) -> (MatrixSizeType, MatrixSizeType);
}

impl<R: SizeIdentifier, C: SizeIdentifier, T: SizeType<R = R, C = C>> Size<R, C> for T {
    fn size_type(&self) -> (MatrixSizeType, MatrixSizeType) {
        (R::IDENT, C::IDENT)
    }
}
