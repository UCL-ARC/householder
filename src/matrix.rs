//! Definition of base classes associated with matrices.
//!

use crate::base_matrix::*;
use crate::iterators::*;
use crate::matrix_traits::*;
use crate::scalar_mult::ScalarMult;
use cauchy::Scalar;
use std::marker::PhantomData;

/// A matrix is a simple enum struct.
/// This can be a base matrix or something
/// representing the sum, product, etc. on
/// matrices.
pub struct Matrix<'a, Item, MatImpl, Layout, Size>(
    MatImpl,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<Size>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>;

impl<'a, Item, MatImpl, Layout, Size> Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    pub fn new(op: MatImpl) -> Self {
        Self(op, PhantomData, PhantomData, PhantomData, PhantomData)
    }

    /// Return a new iterator that iterates through the matrix in memory order
    pub fn iter(
        &self,
    ) -> MatrixIterator<'a, Item, Matrix<Item, MatImpl, Layout, Size>, Layout, Size> {
        MatrixIterator::new(self)
    }
}

impl<'a, Item, MatImpl> Matrix<'a, Item, MatImpl, CLayout, MatrixD>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, CLayout, MatrixD>,
{
    pub fn eval(&self) -> Matrix<Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD> {
        let (rows, cols) = self.dim();
        let nelems = rows * cols;
        let mut res = Matrix::<Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD>::from_dimensions(
            rows, cols,
        );

        unsafe {
            for index in 0..nelems {
                *res.get1d_unchecked_mut(index) = self.get1d_unchecked(index);
            }
        }
        res
    }
}

impl<'a, Item, MatImpl> Matrix<'a, Item, MatImpl, FortranLayout, MatrixD>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, FortranLayout, MatrixD>,
{
    pub fn eval(&self) -> Matrix<Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD> {
        let (rows, cols) = self.dim();
        let nelems = rows * cols;
        let mut res = Matrix::<Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>::from_dimensions_f(
            rows, cols);

        unsafe {
            for index in 0..nelems {
                *res.get1d_unchecked_mut(index) = self.get1d_unchecked(index);
            }
        }
        res
    }
}

impl<'a, Item: Scalar> Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD> {
    /// Create a new matrix with dimensions (rows, cols) using C Layout
    pub fn from_dimensions(rows: usize, cols: usize) -> Self {
        Self::new(DynamicMatrixCLayout::<Item>::new(rows, cols))
    }
}

impl<'a, Item: Scalar> Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD> {
    /// Create a new matrix with dimensions (rows, cols) using Fortran Layout
    pub fn from_dimensions_f(rows: usize, cols: usize) -> Self {
        Self::new(DynamicMatrixFortranLayout::<Item>::new(rows, cols))
    }
}

impl<'a, Item, MatImpl, Layout, Size> Dimensions for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, MatImpl, Layout, Size> SafeRandomAccess for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col)
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        self.0.get1d(index)
    }
}

impl<'a, Item> SafeMutableRandomAccess
    for Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD>
where
    Item: Scalar,
{
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.0.get_mut(row, col)
    }
    #[inline]
    fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get1d_mut(index)
    }
}

impl<'a, Item> SafeMutableRandomAccess
    for Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>
where
    Item: Scalar,
{
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.0.get_mut(row, col)
    }
    #[inline]
    fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get1d_mut(index)
    }
}

impl<'a, Item> UnsafeMutableRandomAccess
    for Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD>
where
    Item: Scalar,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.0.get_unchecked_mut(row, col)
    }
    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get1d_unchecked_mut(index)
    }
}

impl<'a, Item> UnsafeMutableRandomAccess
    for Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>
where
    Item: Scalar,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.0.get_unchecked_mut(row, col)
    }
    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get1d_unchecked_mut(index)
    }
}

impl<'a, Item, MatImpl, Layout, Size> UnsafeRandomAccess for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0.get_unchecked(row, col)
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        self.0.get1d_unchecked(index)
    }
}

impl<'a, Item, MatImpl, Layout, Size> SizeType for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;
}

impl<'a, Item, MatImpl, Layout, Size> LayoutType for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type L = Layout;
}

/// A MatrixRef is like a matrix but holds a reference to an implementation.
pub struct MatrixRef<'a, Item, MatImpl, Layout, Size>(
    &'a MatImpl,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<Size>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>;

impl<'a, Item, MatImpl, Layout, Size> MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    pub fn new(op: &'a MatImpl) -> Matrix<'a, Item, Self, Layout, Size> {
        Matrix::new(Self(op, PhantomData, PhantomData, PhantomData, PhantomData))
    }
}

impl<'a, Item, MatImpl, Layout, Size> Dimensions for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, MatImpl, Layout, Size> SafeRandomAccess
    for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col)
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        self.0.get1d(index)
    }
}

impl<'a, Item, MatImpl, Layout, Size> UnsafeRandomAccess
    for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0.get_unchecked(row, col)
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        self.0.get1d_unchecked(index)
    }
}

impl<'a, Item, MatImpl, Layout, Size> SizeType for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;
}

impl<'a, Item, MatImpl, Layout, Size> LayoutType for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type L = Layout;
}

impl<'a, MatImpl, Layout, Size> std::ops::Mul<Matrix<'a, f64, MatImpl, Layout, Size>> for f64
where
    MatImpl: MatrixTrait<'a, f64, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        f64,
        ScalarMult<'a, f64, Matrix<'a, f64, MatImpl, Layout, Size>, Layout, Size>,
        Layout,
        Size,
    >;

    fn mul(self, rhs: Matrix<'a, f64, MatImpl, Layout, Size>) -> Self::Output {
        Matrix::new(ScalarMult::new(rhs, self))
    }
}

impl<'a, MatImpl, Layout, Size> std::ops::Mul<&'a Matrix<'a, f64, MatImpl, Layout, Size>> for f64
where
    MatImpl: MatrixTrait<'a, f64, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        f64,
        ScalarMult<
            'a,
            f64,
            Matrix<
                'a,
                f64,
                MatrixRef<'a, f64, Matrix<'a, f64, MatImpl, Layout, Size>, Layout, Size>,
                Layout,
                Size,
            >,
            Layout,
            Size,
        >,
        Layout,
        Size,
    >;

    fn mul(self, rhs: &'a Matrix<'a, f64, MatImpl, Layout, Size>) -> Self::Output {
        Matrix::new(ScalarMult::new(MatrixRef::new(rhs), self))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mat;

    #[test]
    fn scalar_mult() {
        let mut mat = mat![f64, (2, 3), CLayout];

        *mat.get_mut(1, 2) = 2.0;

        let res = 5.0 * &mat;

        assert_eq!(res.get(1, 2), 10.0);
    }
}
