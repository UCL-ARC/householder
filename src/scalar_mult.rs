//! Multiplication of a matrix with a scalar
use cauchy::Scalar;
use crate::matrix_traits::*;
use crate::iterators::*;
use std::marker::PhantomData;


pub struct ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>(
    MatImpl,
    Item, 
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<Size>,
    PhantomData<Iter>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>;

impl<'a, Item, MatImpl, Layout, Size, Iter> ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    pub fn new(op: MatImpl, factor: Item) -> Self {
        Self(
            op,
            factor,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<'a, Item, MatImpl, Layout, Size, Iter> Dimensions
    for ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, MatImpl, Layout, Size, Iter> SafeRandomAccess
    for ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    type Output = Item;

    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.1 * self.0.get(row, col)
    }
}

impl<'a, Item, MatImpl, Layout, Size, Iter> UnsafeRandomAccess
    for ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    type Output = Item;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.1 * self.0.get(row, col)
    }
}

impl<'a, Item, MatImpl, Layout, Size, Iter> SizeType
    for ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    type S = Size;

}

impl<'a, Item, MatImpl, Layout, Size, Iter> LayoutType
    for ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    type L = Layout;

}

impl<'a, Item, MatImpl, Layout, Size, Iter> Iterable<'a, Item, ScalarMultIterator<'a, Item, Iter>>
    for ScalarMult<'a, Item, MatImpl, Layout, Size, Iter>
where
    Item: Scalar,
    MatImpl: MatrixTrait<'a, Item, Layout, Size, Iter>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
{
    
    fn iter(&'a self) -> ScalarMultIterator<'a, Item, Iter>{
        ScalarMultIterator::new(self.1, self.0.iter())
    }

}

