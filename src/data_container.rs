//! Containers are simple structures that hold data and allow access to it.

use crate::types::IndexType;
use crate::types::Scalar;

pub trait DataContainer {
    type Item: Scalar;

    /// Access the container unchecked.
    unsafe fn get_unchecked(&self, index: IndexType) -> Self::Item;

    /// Get pointer to data.
    fn get_pointer(&self) -> *const Self::Item;

    /// Return the number of elements in the container.
    fn number_of_elements(&self) -> IndexType;
}

pub trait DataContainerMut: DataContainer {
    /// Access the container mutably unchecked.
    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item;

    /// Get mutable pointer to data.
    fn get_pointer_mut(&mut self) -> *mut Self::Item;
}

/// A container that uses dynamic vectors.
pub struct VectorContainer<Item: Scalar> {
    data: Vec<Item>,
}

/// A container that takes a reference to a slice.
pub struct SliceContainer<'a, Item: Scalar> {
    data: &'a [Item],
}

/// A container that takes a mutable reference to a slice.
pub struct SliceContainerMut<'a, Item: Scalar> {
    data: &'a mut [Item],
}

impl<Item: Scalar> VectorContainer<Item> {
    /// New vector container by specifying the number of elements.
    /// 
    /// The container is initialized with zeros by default.
    pub fn new(nelems: IndexType) -> VectorContainer<Item> {
        VectorContainer::<Item> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); nelems],
        }
    }
}

impl<'a, Item: Scalar> SliceContainer<'a, Item> {
    /// New slice container from a reference.
    pub fn new(slice: &'a [Item]) -> SliceContainer<Item> {
        SliceContainer::<Item> { data: slice }
    }
}

impl<'a, Item: Scalar> SliceContainerMut<'a, Item> {
    /// New mutable slice container from mutable reference.
    pub fn new(slice: &'a mut [Item]) -> SliceContainerMut<Item> {
        SliceContainerMut::<Item> { data: slice }
    }
}

impl<Item: Scalar> DataContainer for VectorContainer<Item> {
    type Item = Item;

    unsafe fn get_unchecked(&self, index: IndexType) -> Self::Item {
        *self.data.get_unchecked(index)
    }

    fn get_pointer(&self) -> *const Self::Item {
        self.data.as_ptr()
    }

    fn number_of_elements(&self) -> IndexType {
        self.data.len()
    }
}

impl<Item: Scalar> DataContainerMut for VectorContainer<Item> {
    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(index)
    }

    fn get_pointer_mut(&mut self) -> *mut Self::Item {
        self.data.as_mut_ptr()
    }
}

impl<'a, Item: Scalar> DataContainer for SliceContainer<'a, Item> {
    type Item = Item;

    unsafe fn get_unchecked(&self, index: IndexType) -> Self::Item {
        *self.data.get_unchecked(index)
    }

    fn get_pointer(&self) -> *const Self::Item {
        self.data.as_ptr()
    }

    fn number_of_elements(&self) -> IndexType {
        self.data.len()
    }
}

impl<'a, Item: Scalar> DataContainer for SliceContainerMut<'a, Item> {
    type Item = Item;

    unsafe fn get_unchecked(&self, index: IndexType) -> Self::Item {
        *self.data.get_unchecked(index)
    }

    fn get_pointer(&self) -> *const Self::Item {
        self.data.as_ptr()
    }

    fn number_of_elements(&self) -> IndexType {
        self.data.len()
    }
}

impl<'a, Item: Scalar> DataContainerMut for SliceContainerMut<'a, Item> {
    unsafe fn get_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(index)
    }

    fn get_pointer_mut(&mut self) -> *mut Self::Item {
        self.data.as_mut_ptr()
    }
}
