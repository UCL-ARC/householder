//! Definition of iterators

//! For built-in types we use the `std::slice::Iter` struct as implementation.
//! Users can provide custom iterators as well, which are packed within an
//! Iterator struct.

use crate::traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub struct MatrixIterator<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
{
    mat: &'a MatImpl,
    nelems: usize,
    index: usize,
    phantom_item: PhantomData<Item>,
    phantom_layout: PhantomData<Layout>,
    phantom_size: PhantomData<Size>,
}

impl<'a, Item, MatImpl, Layout, Size> MatrixIterator<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    Size: SizeIdentifier,
    Layout: LayoutIdentifier,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
{
    pub fn new(mat: &'a MatImpl) -> Self {
        let dim = mat.dim();
        Self {
            mat,
            nelems: dim.0 * dim.1,
            index: 0,
            phantom_item: PhantomData,
            phantom_layout: PhantomData,
            phantom_size: PhantomData,
        }
    }
}

impl<'a, Item, MatImpl, Layout, Size> std::iter::Iterator
    for MatrixIterator<'a, Item, MatImpl, Layout, Size>
where
    Item: Scalar,
    Size: SizeIdentifier,
    Layout: LayoutIdentifier,
    MatImpl: MatrixTrait<'a, Item, Layout, Size>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.nelems {
            return None;
        }

        let res = unsafe { Some(self.mat.get1d_unchecked(self.index)) };
        self.index += 1;

        res
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.nelems - self.index;
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.nelems
    }

}
