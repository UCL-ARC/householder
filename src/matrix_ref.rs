//! A matrix that holds a reference to another matrix.
//!
//! The type defined in this module translates a reference to a matrix into an owned matrix.
//! Why is this necessary? Consider the code `let sum = mat1 + mat2` with `mat1` and `mat2`
//! matrices. The result `sum` is an addition type that now owns `mat1` and `mat2`. But
//! often we do not want `sum` to take ownership of the terms on the right hand side.
//! So we want to write `let sum = &mat1 + &mat2`. But now the type that implements addition
//! needs to hold references to matrices and cannot take ownership anymore. So we need to
//! implement different addition types for each of the cases `mat1 + mat2`, `&mat1 + mat2`,
//! `mat1 + &mat2`, `&mat2 + &mat2`. This would require significant code duplication. The
//! solution is to create a type that turns a reference to a matrix into an owned matrix.
//! This is what [MatrixRef] is doing. It simply takes a reference to a matrix and forwards
//! all matrix operations to the reference. Hence, an expression of the form `&mat1 + mat2` will
//! first be converted into an expression similar to `MatrixRef(&mat1) + mat2`, and then
//! both terms passed onto the addition type, which takes ownership of both terms.

use crate::matrix::Matrix;
use crate::traits::*;
use crate::types::{HScalar, IndexType};
use std::marker::PhantomData;

/// A struct that implements [MatrixTrait] by holding a reference
/// to a matrix and forwarding all matrix operations to the held reference.
pub struct MatrixRef<'a, Item, MatImpl, L, RS, CS>(
    &'a Matrix<Item, MatImpl, L, RS, CS>,
    PhantomData<Item>,
    PhantomData<L>,
    PhantomData<RS>,
    PhantomData<CS>,
)
where
    Item: HScalar,
    L: LayoutType,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    MatImpl: MatrixTrait<Item, L, RS, CS>;

impl<
        'a,
        Item: HScalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > MatrixRef<'a, Item, MatImpl, L, RS, CS>
{
    /// Create a new MatrixRef
    pub fn new(mat: &'a Matrix<Item, MatImpl, L, RS, CS>) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }
}

/// Mutable reference to a Matrix
pub struct MatrixRefMut<'a, Item, MatImpl, L, RS, CS>(
    &'a mut Matrix<Item, MatImpl, L, RS, CS>,
    PhantomData<Item>,
    PhantomData<L>,
    PhantomData<RS>,
    PhantomData<CS>,
)
where
    Item: HScalar,
    L: LayoutType,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    MatImpl: MatrixTrait<Item, L, RS, CS>;

impl<
        'a,
        Item: HScalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > MatrixRefMut<'a, Item, MatImpl, L, RS, CS>
{
    /// Create a new MatrixRefMut
    pub fn new(mat: &'a mut Matrix<Item, MatImpl, L, RS, CS>) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }
}

macro_rules! matrix_ref_traits {
    ($MatrixRefType:ident) => {
        impl<
                'a,
                Item: HScalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutType,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > Layout for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
        {
            type Impl = L;

            #[inline]
            fn layout(&self) -> &Self::Impl {
                self.0.layout()
            }
        }

        impl<
                'a,
                Item: HScalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutType,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > SizeType for $MatrixRefType<'a, Item, MatImpl, L, RS, CS>
        {
            type R = RS;
            type C = CS;
        }

        impl<
                'a,
                Item: HScalar,
                MatImpl: MatrixTrait<Item, L, RS, CS>,
                L: LayoutType,
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
        Item: HScalar,
        MatImpl: MatrixTraitMut<Item, L, RS, CS>,
        L: LayoutType,
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
