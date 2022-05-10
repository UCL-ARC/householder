//! Definition of base classes associated with matrices.
//!

use crate::base_types::*;
//use crate::iterators::*;
use crate::traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub type MatrixFromRef<'a, Item, MatImpl, Layout, RS, CS> = Matrix<
    'a,
    Item,
    MatrixRef<'a, Item, Matrix<'a, Item, MatImpl, Layout, RS, CS>, Layout, RS, CS>,
    Layout,
    RS,
    CS,
>;

pub type ColVectorD<'a, Item> =
    Matrix<'a, Item, DynamicMatrix<Item, VLayout, Dynamic, Fixed1>, VLayout, Dynamic, Fixed1>;
pub type RowVectorD<'a, Item> =
    Matrix<'a, Item, DynamicMatrix<Item, VLayout, Fixed1, Dynamic>, VLayout, Fixed1, Dynamic>;
pub type CMatrixD<'a, Item> =
    Matrix<'a, Item, DynamicMatrix<Item, CLayout, Dynamic, Dynamic>, CLayout, Dynamic, Dynamic>;
pub type FMatrixD<'a, Item> =
    Matrix<'a, Item, DynamicMatrix<Item, FLayout, Dynamic, Dynamic>, FLayout, Dynamic, Dynamic>;

/// A matrix is a simple enum struct.
/// This can be a base matrix or something
/// representing the sum, product, etc. on
/// matrices.
pub struct Matrix<'a, Item, MatImpl, Layout, RS, CS>(
    MatImpl,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<RS>,
    PhantomData<CS>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>;

impl<'a, Item, MatImpl, Layout, RS, CS> Matrix<'a, Item, MatImpl, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    pub fn new(mat: MatImpl) -> Self {
        Self(
            mat,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }

    // /// Return a new iterator that iterates through the matrix in memory order
    // pub fn iter(
    //     &self,
    // ) -> MatrixIterator<'a, Item, Matrix<Item, MatImpl, Layout, Size>, Layout, Size> {
    //     MatrixIterator::new(self)
    // }

    /// Convert a reference to a matrix into an owned matrix.
    ///
    /// The owned matrix itself holds a reference to the original matrix and does
    /// not allocate new memory. This allows handing over matrices to functions
    /// that expect a matrix and not a reference to a matrix.
    pub fn from_ref(
        mat: &'a Matrix<'a, Item, MatImpl, Layout, RS, CS>,
    ) -> MatrixFromRef<'a, Item, MatImpl, Layout, RS, CS> {
        Matrix::new(MatrixRef::new(mat))
    }
}

macro_rules! matrix_traits_impl {
    ($Layout:ty, $RS:ty, $CS:ty) => {
        impl<'a, Item, MatImpl> Matrix<'a, Item, MatImpl, $Layout, $RS, $CS>
        where
            Item: Scalar,
            MatImpl: MatrixTrait<'a, Item, $Layout, $RS, $CS>,
        {
            pub fn eval(
                &self,
            ) -> Matrix<Item, DynamicMatrix<Item, $Layout, $RS, $CS>, $Layout, $RS, $CS> {
                let (rows, cols) = self.dim();
                let nelems = rows * cols;
                let mut res = Matrix::<
                    Item,
                    DynamicMatrix<Item, $Layout, $RS, $CS>,
                    $Layout,
                    $RS,
                    $CS,
                >::from_dimension(rows, cols);

                unsafe {
                    for index in 0..nelems {
                        *res.get1d_unchecked_mut(index) = self.get1d_unchecked(index);
                    }
                }
                res
            }
        }

        impl<'a, Item: Scalar>
            Matrix<'a, Item, DynamicMatrix<Item, $Layout, $RS, $CS>, $Layout, $RS, $CS>
        {
            /// Create a new matrix with dimensions (rows, cols) using C Layout
            pub fn from_dimension(rows: usize, cols: usize) -> Self {
                Self::new(DynamicMatrix::<Item, $Layout, $RS, $CS>::new(rows, cols))
            }
        }

        impl<'a, Item: Scalar> Pointer
            for Matrix<'a, Item, DynamicMatrix<Item, $Layout, $RS, $CS>, $Layout, $RS, $CS>
        {
            type Item = Item;

            fn as_ptr(&self) -> *const Self::Item {
                self.0.as_ptr()
            }
        }

        impl<'a, Item: Scalar> PointerMut
            for Matrix<'a, Item, DynamicMatrix<Item, $Layout, $RS, $CS>, $Layout, $RS, $CS>
        {
            type Item = Item;

            fn as_mut_ptr(&mut self) -> *mut Self::Item {
                self.0.as_mut_ptr()
            }
        }

        impl<'a, Item> SafeMutableRandomAccess
            for Matrix<'a, Item, DynamicMatrix<Item, $Layout, $RS, $CS>, $Layout, $RS, $CS>
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
            for Matrix<'a, Item, DynamicMatrix<Item, $Layout, $RS, $CS>, $Layout, $RS, $CS>
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
    };
}

matrix_traits_impl!(CLayout, Dynamic, Dynamic);
matrix_traits_impl!(FLayout, Dynamic, Dynamic);

impl<'a, Item, MatImpl, Layout, RS, CS> Dimensions for Matrix<'a, Item, MatImpl, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, MatImpl, Layout, RS, CS> SafeRandomAccess
    for Matrix<'a, Item, MatImpl, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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

impl<'a, Item, MatImpl, Layout, RS, CS> UnsafeRandomAccess
    for Matrix<'a, Item, MatImpl, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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

impl<'a, Item, MatImpl, Layout, RS, CS> SizeType for Matrix<'a, Item, MatImpl, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type R = RS;
    type C = CS;
}

impl<'a, Item, MatImpl, Layout, RS, CS> LayoutType<Layout>
    for Matrix<'a, Item, MatImpl, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
}

/// A MatrixRef is like a matrix but holds a reference to an implementation.
pub struct MatrixRef<'a, Item, Mat, Layout, RS, CS>(
    &'a Mat,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<RS>,
    PhantomData<CS>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>;

impl<'a, Item, Mat, Layout, RS, CS> MatrixRef<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    pub fn new(mat: &'a Mat) -> Self {
        Self(
            mat,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<'a, Item, Mat, Layout, RS, CS> Dimensions for MatrixRef<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, Mat, Layout, RS, CS> SafeRandomAccess for MatrixRef<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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

impl<'a, Item, Mat, Layout, RS, CS> UnsafeRandomAccess for MatrixRef<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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

impl<'a, Item, Mat, Layout, RS, CS> SizeType for MatrixRef<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type R = RS;
    type C = CS;
}

impl<'a, Item, Mat, Layout, RS, CS> LayoutType<Layout> for MatrixRef<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
}

// Extra traits for vectors
matrix_traits_impl!(VLayout, Dynamic, Fixed1);
matrix_traits_impl!(VLayout, Fixed1, Dynamic);

impl<'a, Item, Mat, Layout> VectorLength for Matrix<'a, Item, Mat, Layout, Dynamic, Fixed1>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Dynamic, Fixed1>,
    Layout: LayoutIdentifier,
{
    fn len(&self) -> usize {
        self.dim().0
    }
}

impl<'a, Item, Mat, Layout> VectorLength for Matrix<'a, Item, Mat, Layout, Fixed1, Dynamic>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, Fixed1, Dynamic>,
    Layout: LayoutIdentifier,
{
    fn len(&self) -> usize {
        self.dim().1
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mat;
    use crate::col_vec;

    #[test]
    fn scalar_mult_matrix() {
        let mut mat1 = mat![f64, (2, 3), FLayout];
        let mut mat2 = mat![f64, (2, 3), FLayout];

        *mat1.get_mut(1, 2) = 2.0;
        *mat2.get_mut(1, 2) = 3.0;

        let res = 5.0 * &mat1 + mat2;

        assert_eq!(res.get(1, 2), 13.0);
    }

    #[test]
    fn scalar_mult_vector() {
        let mut vec1 = col_vec![f64, 5];
        let mut vec2 = col_vec![f64, 5];

        *vec1.get1d_mut(2) = 2.0;
        *vec2.get1d_mut(2) = 3.0;

        let res = 5.0 * vec1 + &vec2;

        assert_eq!(res.get1d(2), 13.0);
    }


}
