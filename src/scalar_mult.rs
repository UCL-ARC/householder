//! Multiplication of a matrix with a scalar
use cauchy::Scalar;
use crate::matrix_traits::*;
use std::marker::PhantomData;


pub struct ScalarMult<'a, Item, MatImpl, Layout, Size>(
    MatImpl,
    Item, 
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

impl<'a, Item, MatImpl, Layout, Size> ScalarMult<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    pub fn new(op: MatImpl, factor: Item) -> Self {
        Self(
            op,
            factor,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<'a, Item, MatImpl, Layout, Size> Dimensions
    for ScalarMult<'a, Item, MatImpl, Layout, Size>
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
    for ScalarMult<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
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

impl<'a, Item, MatImpl, Layout, Size> UnsafeRandomAccess
    for ScalarMult<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
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

impl<'a, Item, MatImpl, Layout, Size> SizeType
    for ScalarMult<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;

}

impl<'a, Item, MatImpl, Layout, Size> LayoutType
    for ScalarMult<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type L = Layout;

}
