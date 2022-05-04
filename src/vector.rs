//! This module defines the basic data structures for vectors

use crate::matrix_traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

struct Row {}
struct Column {}

pub enum OrientationName {
    ROW,
    COL,
}

pub trait VectorOrientation {}

impl VectorOrientation for Row {}
impl VectorOrientation for Column {}

pub trait OrientationType {
    type O: VectorOrientation;
}

pub trait OrientationTypeHandler<O: VectorOrientation>: OrientationType {
    fn orientation(&self) -> OrientationName;
}

impl<Vec: OrientationType<O = Row>> OrientationTypeHandler<Row> for Vec {
    fn orientation(&self) -> OrientationName {
        OrientationName::ROW
    }
}

impl<Vec: OrientationType<O = Column>> OrientationTypeHandler<Column> for Vec {
    fn orientation(&self) -> OrientationName {
        OrientationName::COL
    }
}

pub trait VectorLength {
    /// Return the length of the vector
    fn len(&self) -> usize;
}

pub trait VectorRandomAccess {
    type Item;

    fn get(&self, index: usize) -> Self::Item;

    unsafe fn get_unchecked(&self, index: usize) -> Self::Item;
}

pub trait VectorRandomAccessMut {
    type Item;

    fn get_mut(&mut self, index: usize) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &Self::Item;
}

pub trait VectorTrait<Item: Scalar, O: VectorOrientation, Size: SizeIdentifier>:
    OrientationType<O = O> + SizeType<S = Size> + VectorLength + VectorRandomAccess
{
}

impl<Item, O, Size, T> VectorTrait<Item, O, Size> for T
where
    Item: Scalar,
    O: VectorOrientation,
    Size: SizeIdentifier,
    T: OrientationType<O = O> + SizeType<S = Size> + VectorLength + VectorRandomAccess,
{
}

pub struct DynamicVector<Item: Scalar, O: OrientationType>(Vec<Item>, PhantomData<O>);

impl<Item: Scalar, O: OrientationType> VectorLength for DynamicVector<Item, O> {

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Item: Scalar, O: OrientationType> VectorRandomAccess for DynamicVector<Item, O> {

    type Item = Item;

    fn get(&self, index: usize) -> Self::Item {
        *self.0.get(index).unwrap()
    }

    unsafe fn get_unchecked(&self, index: usize) -> Self::Item {
        *self.0.get_unchecked(index)
    } 
}

impl<Item: Scalar, O: OrientationType> VectorRandomAccessMut for DynamicVector<Item, O> {

    type Item = Item;

    fn get_mut(&mut self, index: usize) -> &Self::Item {
        self.0.get_mut(index).unwrap()
    }

    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &Self::Item {
        self.0.get_unchecked(index)
    } 
}

impl<Item: Scalar, O: OrientationType> SizeType for DynamicVector<Item, O> {
    type S = VectorD;
}
