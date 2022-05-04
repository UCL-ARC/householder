//! Definition of base classes associated with matrices.
//!

use crate::base_matrix::*;
use crate::iterators::*;
use crate::matrix_traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub type MatrixFromRef<'a, Item, MatImpl, Layout, Size> = Matrix<
    'a,
    Item,
    MatrixRef<'a, Item, Matrix<'a, Item, MatImpl, Layout, Size>, Layout, Size>,
    Layout,
    Size,
>;

pub type CMatrixD<'a, Item> = Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD>;
pub type FMatrixD<'a, Item> = Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>;

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
    pub fn new(mat: MatImpl) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }

    /// Return a new iterator that iterates through the matrix in memory order
    pub fn iter(
        &self,
    ) -> MatrixIterator<'a, Item, Matrix<Item, MatImpl, Layout, Size>, Layout, Size> {
        MatrixIterator::new(self)
    }

    /// Convert a reference to a matrix into an owned matrix.
    ///
    /// The owned matrix itself holds a reference to the original matrix and does
    /// not allocate new memory. This allows handing over matrices to functions
    /// that expect a matrix and not a reference to a matrix.
    pub fn from_ref(
        mat: &'a Matrix<'a, Item, MatImpl, Layout, Size>,
    ) -> MatrixFromRef<'a, Item, MatImpl, Layout, Size> {
        Matrix::new(MatrixRef::new(mat))
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
        let mut res = Matrix::<Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD>::from_dimension(
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
        let mut res = Matrix::<Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>::from_dimension(
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
    pub fn from_dimension(rows: usize, cols: usize) -> Self {
        Self::new(DynamicMatrixCLayout::<Item>::new(rows, cols))
    }
}

impl<'a, Item: Scalar> Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD> {
    /// Create a new matrix with dimensions (rows, cols) using Fortran Layout
    pub fn from_dimension(rows: usize, cols: usize) -> Self {
        Self::new(DynamicMatrixFortranLayout::<Item>::new(rows, cols))
    }
}

impl<'a, Item: Scalar> Pointer for Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD> {
    type Item = Item;

    fn as_ptr(&self) -> *const Self::Item {
        self.0.as_ptr()
    }
}

impl<'a, Item: Scalar> Pointer
    for Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>
{
    type Item = Item;

    fn as_ptr(&self) -> *const Self::Item {
        self.0.as_ptr()
    }
}

impl<'a, Item: Scalar> PointerMut for Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD> {
    type Item = Item;

    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.0.as_mut_ptr()
    }
}

impl<'a, Item: Scalar> PointerMut
    for Matrix<'a, Item, DynamicMatrixFortranLayout<Item>, FortranLayout, MatrixD>
{
    type Item = Item;

    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.0.as_mut_ptr()
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
pub struct MatrixRef<'a, Item, Mat, Layout, Size>(
    &'a Mat,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<Size>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Mat: MatrixTrait<'a, Item, Layout, Size>;

impl<'a, Item, Mat, Layout, Size> MatrixRef<'a, Item, Mat, Layout, Size>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    pub fn new(mat: &'a Mat) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }
}

impl<'a, Item, Mat, Layout, Size> Dimensions for MatrixRef<'a, Item, Mat, Layout, Size>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, Mat, Layout, Size> SafeRandomAccess for MatrixRef<'a, Item, Mat, Layout, Size>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Size>,
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

impl<'a, Item, Mat, Layout, Size> UnsafeRandomAccess for MatrixRef<'a, Item, Mat, Layout, Size>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Size>,
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

impl<'a, Item, Mat, Layout, Size> SizeType for MatrixRef<'a, Item, Mat, Layout, Size>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;
}

impl<'a, Item, Mat, Layout, Size> LayoutType for MatrixRef<'a, Item, Mat, Layout, Size>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type L = Layout;
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mat;

    #[test]
    fn scalar_mult() {
        let mut mat1 = mat![f64, (2, 3), CLayout];
        let mut mat2 = mat![f64, (2, 3), CLayout];

        *mat1.get_mut(1, 2) = 2.0;
        *mat2.get_mut(1, 2) = 3.0;

        let res = 5.0 * &mat1 + mat2;

        assert_eq!(res.get(1, 2), 13.0);
    }
}
