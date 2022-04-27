//! Definition of base classes associated with matrices.
//!

use crate::base_matrix::*;
use crate::matrix_operators::*;
use crate::matrix_traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub enum MemoryLayout {
    C,
    F,
    CUSTOM,
}

/// A matrix is a simple enum struct.
/// This can be a base matrix or something
/// representing the sum, product, etc. on
/// matrices.
pub struct Matrix<Item, T>(PhantomData<Item>, T)
where
    Item: Scalar,
    T: RandomAccess<Item> + Dimensions;

impl<Item, T> Matrix<Item, T>
where
    Item: Scalar,
    T: RandomAccess<Item> + Dimensions,
{
    /// Create a new Matrix instance for an object behaving like a matrix
    pub fn new(op: T) -> Self {
        Matrix::<Item, T>(PhantomData, op)
    }

    /// Evaluate a matrix into a new base matrix
    pub fn eval(&self) -> Matrix<Item, BaseMatrixCLayout<Item>> {
        let (rows, cols) = self.dim();
        let mut res = Matrix::<Item, BaseMatrixCLayout<Item>>::from_dimensions(rows, cols);

        unsafe {
            for row_index in 0..rows {
                for col_index in 0..cols {
                    *res.get_unchecked_mut(row_index, col_index) =
                        self.get_unchecked(row_index, col_index);
                }
            }
        }
        res
    }
}

impl<Item, T> Dimensions for Matrix<Item, T>
where
    Item: Scalar,
    T: RandomAccess<Item> + Dimensions,
{
    fn dim(&self) -> (usize, usize) {
        self.1.dim()
    }
}

impl<Item: Scalar> Matrix<Item, BaseMatrixCLayout<Item>> {
    /// Create a new matrix with dimensions (rows, cols)
    pub fn from_dimensions(rows: usize, cols: usize) -> Self {
        Self::new(BaseMatrixCLayout::<Item>::new(rows, cols))
    }
}

impl<Item: Scalar> Matrix<Item, BaseMatrixFortranLayout<Item>> {
    /// Create a new matrix with dimensions (rows, cols) using Fortran Layout
    pub fn from_dimensions_f(rows: usize, cols: usize) -> Self {
        Self::new(BaseMatrixFortranLayout::<Item>::new(rows, cols))
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for Matrix<Item, BaseMatrixCLayout<Item>> {
    type Output = Item;

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Item {
        self.1.get_mut(row, col)
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for Matrix<Item, BaseMatrixCLayout<Item>> {
    type Output = Item;

    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Item {
        self.1.get_unchecked_mut(row, col)
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for Matrix<Item, BaseMatrixFortranLayout<Item>> {
    type Output = Item;

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Item {
        self.1.get_mut(row, col)
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for Matrix<Item, BaseMatrixFortranLayout<Item>> {
    type Output = Item;

    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Item {
        self.1.get_unchecked_mut(row, col)
    }
}

impl<Item, T> SafeRandomAccess for Matrix<Item, T>
where
    Item: Scalar,
    T: RandomAccess<Item> + Dimensions,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.1.get(row, col)
    }
}

impl<Item, T> UnsafeRandomAccess for Matrix<Item, T>
where
    Item: Scalar,
    T: RandomAccess<Item> + Dimensions,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.1.get_unchecked(row, col)
    }
}

impl<'a, Item, T> std::ops::Mul<Item> for &'a Matrix<Item, T>
where 
    Item: Scalar,
    T: RandomAccess<Item> + Dimensions,
    {
        type Output = Matrix<Item, ScalarMult<'a, Item, Matrix<Item, T>>>;

        fn mul(self, rhs: Item) -> Self::Output {
            Matrix::new(
                ScalarMult::new(rhs, self)
            )
        }

    }

    #[cfg(test)]
    mod test {
    
        use super::*;
        use crate::mat;
    
        #[test]
        pub fn test_scalar_mult() {
            let mut mat = mat![f64, (2, 3)];
            *mat.get_mut(1, 1) = 2.0;
            let prod = &mat * 5.0;
            let result = prod.eval();
    
            assert_eq!(result.get(1, 1), 10.0);
        }
    }
    