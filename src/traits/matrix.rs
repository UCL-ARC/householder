//! Matrix trait
//!
use crate::traits::{
    Dimensions, LayoutIdentifier, LayoutType, RandomAccess, RandomAccessMut, SizeIdentifier,
    SizeType,
};
use crate::types::Scalar;

/// Combined trait that summarizes basic matrix properties
pub trait MatrixTrait<
    Item: Scalar,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
>: RandomAccess<Item = Item> + Dimensions + LayoutType<Layout> + SizeType<R = RS, C = CS>
{
}

/// Combined trait for mutable matrices
pub trait MatrixTraitMut<
    Item: Scalar,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
>: RandomAccessMut<Item = Item> + MatrixTrait<Item, Layout, RS, CS>
{
}

impl<
        Item: Scalar,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        Mat: RandomAccess<Item = Item> + Dimensions + LayoutType<L> + SizeType<R = RS, C = CS>,
    > MatrixTrait<Item, L, RS, CS> for Mat
{
}

impl<
        Item: Scalar,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        Mat: MatrixTrait<Item, L, RS, CS> + RandomAccessMut<Item = Item>,
    > MatrixTraitMut<Item, L, RS, CS> for Mat
{
}
