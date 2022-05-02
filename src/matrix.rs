//! Definition of base classes associated with matrices.
//!

use crate::base_matrix::*;
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
        Self(
            op,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<'a, Item, MatImpl> Matrix<'a, Item, MatImpl, CLayout, MatrixD>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, CLayout, MatrixD>,
{
    // pub fn eval(
    //     &'a self,
    // ) -> Matrix<Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD, CopiedSliceIterator<Item>>
    // {
    //     let (row, col) = self.dim();
    //     let res = {
    //         let mut res =       
    //         Matrix::<
    //         Item,
    //         DynamicMatrixCLayout<Item>,
    //         CLayout,
    //         MatrixD,
    //         CopiedSliceIterator<'static, Item>,
    //     >::from_dimensions(row, col);
    //     for res in res.iter_mut() {
    //         *res = num::cast::<f64, Item>(0.0).unwrap();
    //     }
    //     res
    // };

    //     res
    // }
}

// impl<'a, Item, MatImpl, Iter>
//     Matrix<'a, Item, MatImpl, CLayout, MatrixD, Iter>
// where
//     Item: Scalar,
//     MatImpl: MatrixTrait<'a, Item, CLayout, MatrixD, Iter>,
//     Iter: Iterator<Item = &'a Item>

// {
//     fn eval(&self) -> Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD, std::slice::Iter<'a, Item>> {

//         let dim = self.dim();
//         let mut res = mat![num::cast::<f64, Scalar>(0.0), dim, CLayout];

//     }
// }

impl<'a, Item: Scalar>
    Matrix<'a, Item, DynamicMatrixCLayout<Item>, CLayout, MatrixD>
{
    /// Create a new matrix with dimensions (rows, cols) using C Layout
    pub fn from_dimensions(rows: usize, cols: usize) -> Self {
        Self::new(DynamicMatrixCLayout::<Item>::new(rows, cols))
    }
}

impl<'a, Item: Scalar>
    Matrix<
        'a,
        Item,
        DynamicMatrixFortranLayout<Item>,
        FortranLayout,
        MatrixD
    >
{
    /// Create a new matrix with dimensions (rows, cols) using Fortran Layout
    pub fn from_dimensions_f(rows: usize, cols: usize) -> Self {
        Self::new(DynamicMatrixFortranLayout::<Item>::new(rows, cols))
    }
}

impl<'a, Item, MatImpl, Layout, Size> Dimensions
    for Matrix<'a, Item, MatImpl, Layout, Size>
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
    for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col)
    }
}

impl<'a, Item, MatImpl, Layout, Size> UnsafeRandomAccess
    for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col)
    }
}

impl<'a, Item, MatImpl, Layout, Size> SizeType
    for Matrix<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;
}

impl<'a, Item, MatImpl, Layout, Size> LayoutType
    for Matrix<'a, Item, MatImpl, Layout, Size>
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
        Matrix::new(Self(
            op,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        ))
    }
}

impl<'a, Item, MatImpl, Layout, Size> Dimensions
    for MatrixRef<'a, Item, MatImpl, Layout, Size>
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

    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col)
    }
}

impl<'a, Item, MatImpl, Layout, Size> UnsafeRandomAccess
    for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier {
    type Output = Item;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col)
    }
}

impl<'a, Item, MatImpl, Layout, Size> SizeType
    for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;
}

impl<'a, Item, MatImpl, Layout, Size> LayoutType
    for MatrixRef<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type L = Layout;
}


//     // /// Evaluate a matrix into a new base matrix
//     // pub fn eval(&self) -> Matrix<Item, BaseMatrixCLayout<Item>> {
//     //     let (rows, cols) = self.dim();
//     //     let mut res = Matrix::<Item, BaseMatrixCLayout<Item>>::from_dimensions(rows, cols);

//     //     unsafe {
//     //         for row_index in 0..rows {
//     //             for col_index in 0..cols {
//     //                 *res.get_unchecked_mut(row_index, col_index) =
//     //                     self.get_unchecked(row_index, col_index);
//     //             }
//     //         }
//     //     }
//     //     res
//     // }
// }

// impl<'a, Item, MatImpl, Layout, Size, Iter> LayoutType for Matrix<'a, Item, MatImpl, Layout, Size, Iter>
// where
//     Item: Scalar,
//     MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
//     Layout: LayoutIdentifier,
//     Size: SizeIdentifier,
//     Iter: Iterator<Item = &'a Item>
// {
//     type L = T::L;

//     fn layout_type(&self) -> MemoryLayout {

//     }
// }

// impl<Item: Scalar> Matrix<Item, BaseMatrixFortranLayout<Item>> {
//     /// Create a new matrix with dimensions (rows, cols) using Fortran Layout
//     pub fn from_dimensions_f(rows: usize, cols: usize) -> Self {
//         Self::new(BaseMatrixFortranLayout::<Item>::new(rows, cols))
//     }
// }

// impl<Item: Scalar> SafeMutableRandomAccess for Matrix<Item, BaseMatrixCLayout<Item>> {
//     type Output = Item;

//     #[inline]
//     fn get_mut(&mut self, row: usize, col: usize) -> &mut Item {
//         self.0.get_mut(row, col)
//     }
// }

// impl<Item: Scalar> UnsafeMutableRandomAccess for Matrix<Item, BaseMatrixCLayout<Item>> {
//     type Output = Item;

//     #[inline]
//     unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Item {
//         self.0.get_unchecked_mut(row, col)
//     }
// }

// impl<Item: Scalar> SafeMutableRandomAccess for Matrix<Item, BaseMatrixFortranLayout<Item>> {
//     type Output = Item;

//     #[inline]
//     fn get_mut(&mut self, row: usize, col: usize) -> &mut Item {
//         self.0.get_mut(row, col)
//     }
// }

// impl<Item: Scalar> UnsafeMutableRandomAccess for Matrix<Item, BaseMatrixFortranLayout<Item>> {
//     type Output = Item;

//     #[inline]
//     unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Item {
//         self.0.get_unchecked_mut(row, col)
//     }
// }

// impl<Item, T> SafeRandomAccess for Matrix<Item, T>
// where
//     Item: Scalar,
//     T: RandomAccess<Item> + Dimensions + LayoutType,
// {
//     type Output = Item;

//     #[inline]
//     fn get(&self, row: usize, col: usize) -> Self::Output {
//         self.0.get(row, col)
//     }
// }

// impl<Item, T> UnsafeRandomAccess for Matrix<Item, T>
// where
//     Item: Scalar,
//     T: RandomAccess<Item> + Dimensions + LayoutType,
// {
//     type Output = Item;

//     #[inline]
//     unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
//         self.0.get_unchecked(row, col)
//     }
// }

// impl<'a, Item, T> std::ops::Mul<Item> for &'a Matrix<Item, T>
// where
//     Item: Scalar,
//     T: RandomAccess<Item> + Dimensions + LayoutType,
// {
//     type Output = Matrix<Item, ScalarMult<'a, Item, Matrix<Item, T>>>;

//     fn mul(self, rhs: Item) -> Self::Output {
//         Matrix::new(ScalarMult::new(rhs, self))
//     }
// }

// // Implementation of multiplication with scalars from the left.
// // We cannot just use a generic over scalars here as this is disallowed
// // by the fact that some of the scalar types are defined internally in Rust
// // and our crate is an external package (at least I think that's the issue).
// macro_rules! scalar_mult_reverse {
//     ($scalar:ty) => {
//         impl<'a, T> std::ops::Mul<&'a Matrix<$scalar, T>> for $scalar
//         where
//             T: RandomAccess<$scalar> + Dimensions + LayoutType,
//         {
//             type Output = Matrix<$scalar, ScalarMult<'a, $scalar, Matrix<$scalar, T>>>;

//             fn mul(self, rhs: &'a Matrix<$scalar, T>) -> Self::Output {
//                 // Forward back to the original implementation with scalar
//                 // multiplication from the right.
//                 rhs * self
//             }
//         }
//     };
// }

// scalar_mult_reverse!(f32);
// scalar_mult_reverse!(f64);
// scalar_mult_reverse!(c32);
// scalar_mult_reverse!(c64);

// impl<'a, Item, Mat1, Mat2, Layout> std::ops::Add<&'a Matrix<Item, Mat2>> for &'a Matrix<Item, Mat1>
// where
//     Item: Scalar,
//     Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
//     Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
//     Layout: LayoutIdentifier
// {
//     type Output = Matrix<Item, Add<'a, Item, Matrix<Item, Mat1>, Matrix<Item, Mat2>, Layout>>;

//     fn add(self, rhs: &'a Matrix<Item, Mat2>) -> Self::Output {
//         Matrix::new(Add::new(self, rhs))
//     }
// }

// #[cfg(test)]
// mod test {

//     use super::*;
//     use crate::mat;

//     #[test]
//     pub fn test_scalar_mult() {
//         let mut mat = mat![f64, (2, 3)];
//         *mat.get_mut(1, 1) = 2.0;
//         let prod = 5.0 * &mat;
//         let result = prod.eval();

//         assert_eq!(result.get(1, 1), 10.0);
//     }

//     #[test]
//     pub fn test_layout() {
//         let mat = mat![f64, (2, 3)];

//         assert_eq!(mat.layout(), MemoryLayout::C);
//     }

//     #[test]
//     pub fn test_addition() {
//         let mut mat1 = mat![f64, (2, 3)];
//         let mut mat2 = mat![f64, (2, 3)];

//         *mat1.get_mut(1, 2) = 3.0;
//         *mat2.get_mut(1, 2) = 4.0;

//         let prod = 5.0 * &mat1;

//         let mat = &prod + &mat2;

//         assert_eq!(mat.get(1, 2), 19.0);

//     }
// }

impl<'a, MatImpl, Layout, Size> std::ops::Mul<Matrix<'a, f64, MatImpl, Layout, Size>>
    for f64
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

impl<'a, MatImpl, Layout, Size>
    std::ops::Mul<&'a Matrix<'a, f64, MatImpl, Layout, Size>> for f64
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
                MatrixRef<
                    'a,
                    f64,
                    Matrix<'a, f64, MatImpl, Layout, Size>,
                    Layout,
                    Size,
                >,
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
