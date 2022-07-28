//! Matrix trait
//!
use crate::traits::{
    LayoutType, Layout, RandomAccess, RandomAccessMut, SizeIdentifier,
    SizeType,
};
use crate::types::Scalar;

/// Combined trait that summarizes basic matrix properties
pub trait MatrixTrait<
    Item: Scalar,
    L: LayoutType,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
>:
    RandomAccess<Item = Item> + Layout<Impl=L> + SizeType<R = RS, C = CS>
{
}

/// Combined trait for mutable matrices
pub trait MatrixTraitMut<
    Item: Scalar,
    L: LayoutType,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
>: RandomAccessMut<Item = Item> + MatrixTrait<Item, L, RS, CS>
{
}

impl<
        Item: Scalar,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        Mat: RandomAccess<Item = Item> + Layout<Impl=L> + SizeType<R = RS, C = CS>,
    > MatrixTrait<Item, L, RS, CS> for Mat
{
}

impl<
        Item: Scalar,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        Mat: MatrixTrait<Item, L, RS, CS> + RandomAccessMut<Item = Item>,
    > MatrixTraitMut<Item, L, RS, CS> for Mat
{
}
