//! Multiplication of a matrix with a scalar
use crate::matrix::*;
use crate::matrix_ref::MatrixRef;
use crate::traits::*;
use crate::types::{c32, c64, IndexType, Scalar};
use std::marker::PhantomData;

pub type ScalarProdMat<Item, MatImpl, L, RS, CS> =
    Matrix<Item, ScalarMult<Item, MatImpl, L, RS, CS>, L, RS, CS>;

pub struct ScalarMult<Item, MatImpl, L, RS, CS>(
    Matrix<Item, MatImpl, L, RS, CS>,
    Item,
    PhantomData<Item>,
    PhantomData<L>,
    PhantomData<RS>,
    PhantomData<CS>,
)
where
    Item: Scalar,
    L: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    MatImpl: MatrixTrait<Item, L, RS, CS>;

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > ScalarMult<Item, MatImpl, L, RS, CS>
{
    pub fn new(mat: Matrix<Item, MatImpl, L, RS, CS>, scalar: Item) -> Self {
        Self(
            mat,
            scalar,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > LayoutType<L> for ScalarMult<Item, MatImpl, L, RS, CS>
{
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > SizeType for ScalarMult<Item, MatImpl, L, RS, CS>
{
    type R = RS;
    type C = CS;
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Dimensions for ScalarMult<Item, MatImpl, L, RS, CS>
{
    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        self.0.dim()
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.0.number_of_elements()
    }
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Stride for ScalarMult<Item, MatImpl, L, RS, CS>
{
    #[inline]
    fn row_stride(&self) -> IndexType {
        self.0.row_stride()
    }

    #[inline]
    fn column_stride(&self) -> IndexType {
        self.0.column_stride()
    }
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccess for ScalarMult<Item, MatImpl, L, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
        self.1 * self.0.get_unchecked(row, col)
    }

    #[inline]
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
        self.1 * self.0.get1d_unchecked(index)
    }
}

macro_rules! scalar_mult_impl {
    ($Scalar:ty) => {
        impl<
                MatImpl: MatrixTrait<$Scalar, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > std::ops::Mul<Matrix<$Scalar, MatImpl, L, RS, CS>> for $Scalar
        {
            type Output = ScalarProdMat<$Scalar, MatImpl, L, RS, CS>;

            fn mul(self, rhs: Matrix<$Scalar, MatImpl, L, RS, CS>) -> Self::Output {
                Matrix::new(ScalarMult::new(rhs, self))
            }
        }

        impl<
                'a,
                MatImpl: MatrixTrait<$Scalar, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > std::ops::Mul<&'a Matrix<$Scalar, MatImpl, L, RS, CS>> for $Scalar
        {
            type Output =
                ScalarProdMat<$Scalar, MatrixRef<'a, $Scalar, MatImpl, L, RS, CS>, L, RS, CS>;

            fn mul(self, rhs: &'a Matrix<$Scalar, MatImpl, L, RS, CS>) -> Self::Output {
                ScalarProdMat::new(ScalarMult::new(Matrix::from_ref(rhs), self))
            }
        }

        impl<
                MatImpl: MatrixTrait<$Scalar, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > std::ops::Mul<$Scalar> for Matrix<$Scalar, MatImpl, L, RS, CS>
        {
            type Output = Matrix<$Scalar, ScalarMult<$Scalar, MatImpl, L, RS, CS>, L, RS, CS>;

            fn mul(self, rhs: $Scalar) -> Self::Output {
                Matrix::new(ScalarMult::new(self, rhs))
            }
        }

        impl<
                'a,
                MatImpl: MatrixTrait<$Scalar, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > std::ops::Mul<$Scalar> for &'a Matrix<$Scalar, MatImpl, L, RS, CS>
        {
            type Output =
                ScalarProdMat<$Scalar, MatrixRef<'a, $Scalar, MatImpl, L, RS, CS>, L, RS, CS>;

            fn mul(self, rhs: $Scalar) -> Self::Output {
                ScalarProdMat::new(ScalarMult::new(Matrix::from_ref(self), rhs))
            }
        }
    };
}

scalar_mult_impl!(f32);
scalar_mult_impl!(f64);
scalar_mult_impl!(c32);
scalar_mult_impl!(c64);

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn scalar_mult() {
        let mut mat = MatrixD::<f64, CLayout>::from_zeros(2, 3);

        *mat.get_mut(1, 2) = 5.0;

        let res = 2.0 * mat;
        let res = res.eval();

        assert_eq!(res.get(1, 2), 10.0);
    }
}
