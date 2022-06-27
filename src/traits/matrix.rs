//! Matrix trait
//!
use crate::types::Scalar;
use crate::traits::{
    Dimensions, LayoutIdentifier, LayoutType, RandomAccess, RandomAccessMut, SizeIdentifier,
    SizeType,
};

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
pub trait MatrixMutTrait<
    Item: Scalar,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
>:
    RandomAccessMut<Item = Item> + MatrixTrait<Item, Layout, RS, CS>
{
}
