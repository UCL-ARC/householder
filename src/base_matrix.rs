//! Definition of a base matrix.
//! 
//! A base matrix is an implementation of a matrix that
//! is directly associated with a [DataContainer] (typically a memory region).
//! 
//! The user should never interact with [BaseMatrix] directly. Rather, the 
//! relevant user type is a [GenericBaseMatrix](crate::matrix::GenericBaseMatrix),
//! which is a [Matrix](crate::matrix::Matrix) that forwards call to the 
//! [BaseMatrix] implementation.
//! 
use crate::data_container::{DataContainer, DataContainerMut};
use crate::layouts::*;
use crate::traits::*;
use crate::types::{IndexType, Scalar};
use std::marker::PhantomData;

pub struct BaseMatrix<
    Item: Scalar,
    Data: DataContainer<Item = Item>,
    L: LayoutType,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
> {
    data: Data,
    layout: L,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
}

impl<
        Item: Scalar,
        Data: DataContainer<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > BaseMatrix<Item, Data, L, RS, CS>
{
    pub fn new(data: Data, layout: L) -> Self {
        assert!(
            layout.number_of_elements() <= data.number_of_elements(),
            "Number of elements in data: {}. But layout number of elements is {})",
            data.number_of_elements(),
            layout.number_of_elements(),
        );
        BaseMatrix::<Item, Data, L, RS, CS> {
            data,
            layout,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<
        Item: Scalar,
        Data: DataContainer<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > BaseMatrix<Item, Data, L, RS, CS>
{
    #[inline]
    pub fn get_pointer(&self) -> *const Item {
        self.data.get_pointer()
    }

    #[inline]
    pub fn get_slice(&self, first: IndexType, last: IndexType) -> &[Item] {
        self.data.get_slice(first, last)
    }
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > BaseMatrix<Item, Data, L, RS, CS>
{
    #[inline]
    pub fn get_pointer_mut(&mut self) -> *mut Item {
        self.data.get_pointer_mut()
    }

    #[inline]
    pub fn get_slice_mut(&mut self, first: IndexType, last: IndexType) -> &mut [Item] {
        self.data.get_slice_mut(first, last)
    }
}

impl<
        Item: Scalar,
        Data: DataContainer<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Layout for BaseMatrix<Item, Data, L, RS, CS>
{
    type Impl = L;

    #[inline]
    fn layout(&self) -> &Self::Impl {
        &self.layout
    }
}

impl<
        Item: Scalar,
        Data: DataContainer<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > SizeType for BaseMatrix<Item, Data, L, RS, CS>
{
    type R = RS;
    type C = CS;
}

impl<
        Item: Scalar,
        Data: DataContainer<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccess for BaseMatrix<Item, Data, L, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
        self.data
            .get_unchecked(self.layout.convert_2d_raw(row, col))
    }

    #[inline]
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
        self.data.get_unchecked(self.layout.convert_1d_raw(index))
    }
}

macro_rules! vector_length {
    ($Layout:ty) => {
        impl<
                Item: Scalar,
                Data: DataContainer<Item = Item>,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > BaseMatrix<Item, Data, $Layout, RS, CS>
        {
            pub fn length(&self) -> IndexType {
                self.data.number_of_elements()
            }
        }
    };
}

vector_length!(RowVector);

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for BaseMatrix<Item, Data, L, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item {
        self.data
            .get_unchecked_mut(self.layout.convert_2d_raw(row, col))
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.data
            .get_unchecked_mut(self.layout.convert_1d_raw(index))
    }
}