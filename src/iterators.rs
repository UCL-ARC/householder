//! Definition of iterators

//! For built-in types we use the `std::slice::Iter` struct as implementation.
//! Users can provide custom iterators as well, which are packed within an
//! Iterator struct.

use cauchy::Scalar;
use std::marker::PhantomData;

/// The base iterator type
pub type CopiedSliceIterator<'a, Item> = std::iter::Copied<std::slice::Iter<'a, Item>>;

/// The base iterator type for mutable access
pub type SliceIteratorMut<'a, Item> = std::slice::IterMut<'a, Item>;

pub struct ScalarMultIterator<'a, Item: Scalar, Iter: Iterator<Item = Item>> {
    factor: Item,
    iter: Iter,
    phantom_data: PhantomData<&'a ()>,
}

impl<'a, Item: Scalar, Iter: Iterator<Item = Item>> ScalarMultIterator<'a, Item, Iter> {
    pub fn new(factor: Item, iter: Iter) -> Self {
        ScalarMultIterator {
            factor,
            iter,
            phantom_data: PhantomData,
        }
    }
}

impl<'a, Item: Scalar, Iter: Iterator<Item = Item>> Iterator
    for ScalarMultIterator<'a, Item, Iter>
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|value| self.factor * value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn last(self) -> Option<Self::Item> {
        let factor = self.factor;
        self.iter.last().map(|value| factor * value)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth(n).map(|value| self.factor * value)
    }
}
