//! Definition of operators on matrices
use crate::matrix_traits::*;
use cauchy::Scalar;

/// Container for multiplication with a scalar
pub struct ScalarMult<'a, Item, Mat>(Item, &'a Mat)
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions;

impl<'a, Item, Mat> ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions,
{
    /// Create a new instance of Scalar Multiplication
    pub fn new(factor: Item, mat: &'a Mat) -> Self {
        ScalarMult(factor, mat)
    }
}

impl<'a, Item, Mat> SafeRandomAccess for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0 * self.1.get(row, col)
    }
}

impl<'a, Item, Mat> UnsafeRandomAccess for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0 * self.1.get_unchecked(row, col)
    }
}

impl<'a, Item, Mat> Dimensions for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions,
{
    fn dim(&self) -> (usize, usize) {
        self.1.dim()
    }
}

