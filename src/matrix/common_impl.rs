//! Implementation of matrix traits and methods

use crate::base_matrix::BaseMatrix;
use crate::data_container::{ArrayContainer, DataContainer, DataContainerMut};
use crate::matrix::{Matrix, MatrixD};
use crate::traits::*;
use crate::types::{IndexType, Scalar};

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > LayoutType<L> for Matrix<Item, MatImpl, L, RS, CS>
{
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > SizeType for Matrix<Item, MatImpl, L, RS, CS>
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
    > Dimensions for Matrix<Item, MatImpl, L, RS, CS>
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
    > Stride for Matrix<Item, MatImpl, L, RS, CS>
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
    > UnsafeRandomAccess for Matrix<Item, MatImpl, L, RS, CS>
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

impl<
        Item: Scalar,
        MatImpl: MatrixTraitMut<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for Matrix<Item, MatImpl, L, RS, CS>
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

macro_rules! eval_dynamic_matrix {
    ($L:ident) => {
        impl<Item: Scalar, MatImpl: MatrixTrait<Item, $L, Dynamic, Dynamic>>
            Matrix<Item, MatImpl, $L, Dynamic, Dynamic>
        {
            pub fn eval(&self) -> MatrixD<Item, $L> {
                let dim = self.dim();
                let mut result = MatrixD::<Item, $L>::from_zeros(dim.0, dim.1);
                for index in 0..self.number_of_elements() {
                    unsafe { *result.get1d_unchecked_mut(index) = self.get1d_unchecked(index) };
                }
                result
            }
        }
    };
}

macro_rules! eval_fixed_matrix {
    ($L:ident, $RS:ty, $CS:ty) => {
        impl<Item: Scalar, MatImpl: MatrixTrait<Item, $L, $RS, $CS>>
            Matrix<Item, MatImpl, $L, $RS, $CS>
        {
            pub fn eval(
                &self,
            ) -> Matrix<
                Item,
                BaseMatrix<Item, ArrayContainer<Item, { <$RS>::N * <$CS>::N }>, $L, $RS, $CS>,
                $L,
                $RS,
                $CS,
            > {
                let mut result = Matrix::<
                    Item,
                    BaseMatrix<Item, ArrayContainer<Item, { <$RS>::N * <$CS>::N }>, $L, $RS, $CS>,
                    $L,
                    $RS,
                    $CS,
                >::from_zeros();
                for index in 0..self.number_of_elements() {
                    unsafe { *result.get1d_unchecked_mut(index) = self.get1d_unchecked(index) };
                }
                result
            }
        }
    };
}

eval_dynamic_matrix!(CLayout);
eval_dynamic_matrix!(FLayout);

eval_fixed_matrix!(CLayout, Fixed2, Fixed2);
eval_fixed_matrix!(CLayout, Fixed3, Fixed2);
eval_fixed_matrix!(CLayout, Fixed2, Fixed3);
eval_fixed_matrix!(CLayout, Fixed3, Fixed3);

eval_fixed_matrix!(FLayout, Fixed2, Fixed2);
eval_fixed_matrix!(FLayout, Fixed3, Fixed2);
eval_fixed_matrix!(FLayout, Fixed2, Fixed3);
eval_fixed_matrix!(FLayout, Fixed3, Fixed3);

impl<
        Item: Scalar,
        Data: DataContainer<Item = Item>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Matrix<Item, BaseMatrix<Item, Data, L, RS, CS>, L, RS, CS>
{
    #[inline]
    pub fn get_pointer(&self) -> *const Item {
        self.0.get_pointer()
    }

    #[inline]
    pub fn get_slice(&self, first: IndexType, last: IndexType) -> &[Item] {
        self.0.get_slice(first, last)
    }
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Matrix<Item, BaseMatrix<Item, Data, L, RS, CS>, L, RS, CS>
{
    #[inline]
    pub fn get_pointer_mut(&mut self) -> *mut Item {
        self.0.get_pointer_mut()
    }

    #[inline]
    pub fn get_slice_mut(&mut self, first: IndexType, last: IndexType) -> &mut [Item] {
        self.0.get_slice_mut(first, last)
    }
}
