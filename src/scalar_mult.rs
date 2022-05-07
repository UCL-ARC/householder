//! Multiplication of a matrix with a scalar
use crate::matrix::*;
use crate::traits::*;
use cauchy::{Scalar, c32, c64};
use std::marker::PhantomData;

pub struct ScalarMult<'a, Item, Mat, Layout, RS, CS>(
    Mat,
    Item,
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

impl<'a, Item, Mat, Layout, RS, CS> ScalarMult<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    pub fn new(mat: Mat, factor: Item) -> Self {
        Self(
            mat,
            factor,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<'a, Item, Mat, Layout, RS, CS> Dimensions for ScalarMult<'a, Item, Mat, Layout, RS, CS>
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

impl<'a, Item, Mat, Layout, RS, CS> SafeRandomAccess
    for ScalarMult<'a, Item, Mat, Layout, RS, CS>
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
        self.1 * self.0.get(row, col)
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        self.1 * self.0.get1d(index)
    }
}

impl<'a, Item, Mat, Layout, RS, CS> UnsafeRandomAccess
    for ScalarMult<'a, Item, Mat, Layout, RS, CS>
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
        self.1 * self.0.get_unchecked(row, col)
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        self.1 * self.0.get1d_unchecked(index)
    }
}

impl<'a, Item, Mat, Layout, RS, CS> SizeType for ScalarMult<'a, Item, Mat, Layout, RS, CS>
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

impl<'a, Item, Mat, Layout, RS, CS> LayoutType<Layout> for ScalarMult<'a, Item, Mat, Layout, RS, CS>
where
    Item: Scalar,
    Mat: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{}

macro_rules! scalar_mult_impl {
    ($Scalar:ty) => {
        impl<'a, MatImpl, Layout, RS, CS> std::ops::Mul<Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>>
            for $Scalar
        where
            MatImpl: MatrixTrait<'a, $Scalar, Layout, RS, CS>,
            Layout: LayoutIdentifier,
            RS: SizeIdentifier,
            CS: SizeIdentifier,
        {
            type Output = Matrix<
                'a,
                $Scalar,
                ScalarMult<'a, $Scalar, Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>, Layout, RS, CS>,
                Layout,
                RS,
                CS,
            >;

            fn mul(self, rhs: Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>) -> Self::Output {
                Matrix::new(ScalarMult::new(rhs, self))
            }
        }

        impl<'a, MatImpl, Layout, RS, CS> std::ops::Mul<$Scalar>
            for Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>
        where
            MatImpl: MatrixTrait<'a, $Scalar, Layout, RS, CS>,
            Layout: LayoutIdentifier,
            RS: SizeIdentifier,
            CS: SizeIdentifier,
            {
            type Output = Matrix<
                'a,
                $Scalar,
                ScalarMult<'a, $Scalar, Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>, Layout, RS, CS>,
                Layout,
                RS, CS,
            >;

            fn mul(self, rhs: $Scalar) -> Self::Output {
                Matrix::new(ScalarMult::new(self, rhs))
            }
        }

        impl<'a, MatImpl, Layout, RS, CS> std::ops::Mul<&'a Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>>
            for $Scalar
        where
            MatImpl: MatrixTrait<'a, $Scalar, Layout, RS, CS>,
            Layout: LayoutIdentifier,
            RS: SizeIdentifier,
            CS: SizeIdentifier,
            {
            type Output = Matrix<
                'a,
                $Scalar,
                ScalarMult<
                    'a,
                    $Scalar,
                    MatrixFromRef<'a, $Scalar, MatImpl, Layout, RS, CS>,
                    Layout,
                    RS,
                    CS
                >,
                Layout,
                RS,
                CS
            >;

            fn mul(self, rhs: &'a Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>) -> Self::Output {
                Matrix::new(ScalarMult::new(Matrix::from_ref(rhs), self))
            }
        }

        impl<'a, MatImpl, Layout, RS, CS> std::ops::Mul<$Scalar>
            for &'a Matrix<'a, $Scalar, MatImpl, Layout, RS, CS>
        where
            MatImpl: MatrixTrait<'a, $Scalar, Layout, RS, CS>,
            Layout: LayoutIdentifier,
            RS: SizeIdentifier,
            CS: SizeIdentifier,
            {
            type Output = Matrix<
                'a,
                $Scalar,
                ScalarMult<
                    'a,
                    $Scalar,
                    MatrixFromRef<'a, $Scalar, MatImpl, Layout, RS, CS>,
                    Layout,
                    RS,
                    CS,
                >,
                Layout,
                RS,
                CS
            >;

            fn mul(self, rhs: $Scalar) -> Self::Output {
                Matrix::new(ScalarMult::new(Matrix::from_ref(self), rhs))
            }
        }
    };
}

scalar_mult_impl!(f32);
scalar_mult_impl!(f64);
scalar_mult_impl!(c32);
scalar_mult_impl!(c64);


// impl<'a, MatImpl, Layout, Size> std::ops::Mul<Matrix<'a, f64, MatImpl, Layout, Size>> for f64
// where
//     MatImpl: MatrixTrait<'a, f64, Layout, Size>,
//     Layout: LayoutIdentifier,
//     Size: SizeIdentifier,
// {
//     type Output = Matrix<
//         'a,
//         f64,
//         ScalarMult<'a, f64, Matrix<'a, f64, MatImpl, Layout, Size>, Layout, Size>,
//         Layout,
//         Size,
//     >;

//     fn mul(self, rhs: Matrix<'a, f64, MatImpl, Layout, Size>) -> Self::Output {
//         Matrix::new(ScalarMult::new(rhs, self))
//     }
// }

// impl<'a, MatImpl, Layout, Size> std::ops::Mul<f64> for Matrix<'a, f64, MatImpl, Layout, Size>
// where
//     MatImpl: MatrixTrait<'a, f64, Layout, Size>,
//     Layout: LayoutIdentifier,
//     Size: SizeIdentifier,
// {
//     type Output = Matrix<
//         'a,
//         f64,
//         ScalarMult<'a, f64, Matrix<'a, f64, MatImpl, Layout, Size>, Layout, Size>,
//         Layout,
//         Size,
//     >;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Matrix::new(ScalarMult::new(self, rhs))
//     }
// }

// impl<'a, MatImpl, Layout, Size> std::ops::Mul<&'a Matrix<'a, f64, MatImpl, Layout, Size>> for f64
// where
//     MatImpl: MatrixTrait<'a, f64, Layout, Size>,
//     Layout: LayoutIdentifier,
//     Size: SizeIdentifier,
// {
//     type Output = Matrix<
//         'a,
//         f64,
//         ScalarMult<
//             'a,
//             f64,
//             Matrix<
//                 'a,
//                 f64,
//                 MatrixRef<'a, f64, Matrix<'a, f64, MatImpl, Layout, Size>, Layout, Size>,
//                 Layout,
//                 Size,
//             >,
//             Layout,
//             Size,
//         >,
//         Layout,
//         Size,
//     >;

//     fn mul(self, rhs: &'a Matrix<'a, f64, MatImpl, Layout, Size>) -> Self::Output {
//         Matrix::new(ScalarMult::new(MatrixRef::new(rhs), self))
//     }
// }

// impl<'a, MatImpl, Layout, Size> std::ops::Mul<f64> for &'a Matrix<'a, f64, MatImpl, Layout, Size>
// where
//     MatImpl: MatrixTrait<'a, f64, Layout, Size>,
//     Layout: LayoutIdentifier,
//     Size: SizeIdentifier,
// {
//     type Output = Matrix<
//         'a,
//         f64,
//         ScalarMult<
//             'a,
//             f64,
//             Matrix<
//                 'a,
//                 f64,
//                 MatrixRef<'a, f64, Matrix<'a, f64, MatImpl, Layout, Size>, Layout, Size>,
//                 Layout,
//                 Size,
//             >,
//             Layout,
//             Size,
//         >,
//         Layout,
//         Size,
//     >;

//     fn mul(self, rhs: f64) -> Self::Output {
//         Matrix::new(ScalarMult::new(MatrixRef::new(self), rhs))
//     }
// }

