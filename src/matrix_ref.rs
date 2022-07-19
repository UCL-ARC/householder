// A matrix that holds a reference to another matrix.

use crate::matrix::Matrix;
use crate::traits::*;
use crate::types::{IndexType, Scalar};
use std::marker::PhantomData;

pub struct MatrixRef<'a, Item, MatImpl, L, RS, CS>(
    &'a Matrix<Item, MatImpl, L, RS, CS>,
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
        'a,
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > MatrixRef<'a, Item, MatImpl, L, RS, CS>
{
    pub fn new(mat: &'a Matrix<Item, MatImpl, L, RS, CS>) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }

}

pub struct MatrixRefMut<'a, Item, MatImpl, L, RS, CS>(
    &'a mut Matrix<Item, MatImpl, L, RS, CS>,
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
        'a,
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > MatrixRefMut<'a, Item, MatImpl, L, RS, CS>
{
    pub fn new(mat: &'a mut Matrix<Item, MatImpl, L, RS, CS>) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }
}

macro_rules! matrix_ref_traits {
    ($MatrixRefType:ident) => {
        impl<
                'a,
                Item: Scalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > LayoutType<L> for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
        {
        }

        impl<
                'a,
                Item: Scalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > SizeType for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
        {
            type R = RS;
            type C = CS;
        }

        impl<
                'a,
                Item: Scalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > Dimensions for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
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
                'a,
                Item: Scalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > Stride for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
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
                'a,
                Item: Scalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > UnsafeRandomAccess for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
        {
            type Item = Item;

            #[inline]
            unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
                self.0.get_unchecked(row, col)
            }

            #[inline]
            unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
                self.0.get1d_unchecked(index)
            }
        }
    };
}

matrix_ref_traits!(MatrixRef);
matrix_ref_traits!(MatrixRefMut);

impl<
        'a,
        Item: Scalar,
        MatImpl: MatrixTraitMut<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for MatrixRefMut<'a, Item, MatImpl, L, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item {
        self.0.get_unchecked_mut(row, col)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.0.get1d_unchecked_mut(index)
    }
}
