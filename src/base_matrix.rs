//! The base matrix data types
use cauchy::Scalar;
use crate::matrix_traits::*;

/// Base matrix with C Layout
pub struct BaseMatrixCLayout<Item: Scalar> {
    data: Vec<Item>,
    dim: (usize, usize),
}

/// Base matrix with Fortran Layout
pub struct BaseMatrixFortranLayout<Item: Scalar> {
    data: Vec<Item>,
    dim: (usize, usize),
}

impl<Item: Scalar> BaseMatrixCLayout<Item> {

    // New C Layout base matrix with dimensions (rows, cols)
    pub fn new(rows: usize, cols: usize) -> Self {
        BaseMatrixCLayout::<Item> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
            dim: (rows, cols)
        }
    }

}

impl<Item: Scalar> BaseMatrixFortranLayout<Item> {

    // New Fortran Layout base matrix with dimensions (rows, cols)
    pub fn new(rows: usize, cols: usize) -> Self {
        BaseMatrixFortranLayout::<Item> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
            dim: (rows, cols)
        }
    }

}


impl<Item: Scalar> CLayout for BaseMatrixCLayout<Item> {}

impl<Item: Scalar> FortranLayout for BaseMatrixFortranLayout<Item> {}

impl<Item: Scalar> Dimensions for BaseMatrixCLayout<Item> {
    fn dim(&self) -> (usize, usize) {
        self.dim
    }
}

impl<Item: Scalar> Dimensions for BaseMatrixFortranLayout<Item> {
    fn dim(&self) -> (usize, usize) {
        self.dim
    }
}

impl<Item: Scalar> SafeRandomAccess for BaseMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.data.get(row * self.dim.1 + col).unwrap().clone()
    }
}

impl<Item: Scalar> UnsafeRandomAccess for BaseMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.data.get_unchecked(row * self.dim.1 + col).clone()
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for BaseMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_mut(row * self.dim.1 + col).unwrap()
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for BaseMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(row * self.dim.1 + col)
    }
}


impl<Item: Scalar> UnsafeRandomAccess for BaseMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.data.get_unchecked(col * self.dim.0 + row).clone()
    }
}

impl<Item: Scalar> SafeRandomAccess for BaseMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.data.get(col * self.dim.0 + row).unwrap().clone()
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for BaseMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(col * self.dim.0 + row)
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for BaseMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_mut(col * self.dim.0 + row).unwrap()
    }
}
