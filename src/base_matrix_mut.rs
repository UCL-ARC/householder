//! The base matrix data types
use crate::traits::*;
use crate::types::{IndexType, Scalar};
use crate::data_container::DataContainerMut;
use std::marker::PhantomData;

pub struct BaseMatrixMut<
    Item: Scalar,
    Data: DataContainerMut<Item = Item>,
    L: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
> {
    data: Data,
    dim: (IndexType, IndexType),
    stride: (IndexType, IndexType),
    phantom_layout: PhantomData<L>,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > BaseMatrixMut<Item, Data, CLayout, RS, CS>
{
    /// New dynamic matrix with dimensions (rows, cols)
    pub fn new(data: Data, dim: (IndexType, IndexType)) -> Self {
        BaseMatrixMut::<Item, Data, CLayout, RS, CS> {
            data,
            dim,
            stride: (dim.1, 1),
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > BaseMatrixMut<Item, Data, FLayout, RS, CS>
{
    /// New dynamic matrix with dimensions (rows, cols)
    pub fn new(data: Data, dim: (IndexType, IndexType)) -> Self {
        BaseMatrixMut::<Item, Data, FLayout, RS, CS> {
            data,
            dim,
            stride: (1, dim.0),
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > BaseMatrixMut<Item, Data, CustomLayout, RS, CS>
{
    /// New dynamic matrix with dimensions (rows, cols)
    pub fn new(data: Data, dim: (IndexType, IndexType), stride: (IndexType, IndexType)) -> Self {
        BaseMatrixMut::<Item, Data, CustomLayout, RS, CS> {
            data,
            dim,
            stride,
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}



impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Stride for BaseMatrixMut<Item, Data, L, RS, CS>
{
    #[inline]
    fn row_stride(&self) -> IndexType {
        self.stride.0
    }

    #[inline]
    fn column_stride(&self) -> IndexType {
        self.stride.1
    }
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > SizeType for BaseMatrixMut<Item, Data, L, RS, CS>
{
    type R = RS;
    type C = CS;
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > LayoutType<L> for BaseMatrixMut<Item, Data, L, RS, CS>
{
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Dimensions for BaseMatrixMut<Item, Data, L, RS, CS>
{
    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        self.dim
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim.0 * self.dim.1
    }
}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccess for BaseMatrixMut<Item, Data, CLayout, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
        self.data.get_unchecked(row * self.dim.1 + col)
    }

    #[inline]
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
        self.data.get_unchecked(index)    
    }

}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccess for BaseMatrixMut<Item, Data, FLayout, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
        self.data.get_unchecked(col * self.dim.0 + row)
    }

    #[inline]
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
        self.data.get_unchecked(index)    
    }

}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccess for BaseMatrixMut<Item, Data, CustomLayout, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
        self.data.get_unchecked(self.stride.0 * row + self.stride.1 * col)
    }

    #[inline]
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
        let row =   index / self.dim.1;
        let col = index % self.dim.1;

        self.get_unchecked(row, col)
    }

}

// - Unsafe Mutable access

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for BaseMatrixMut<Item, Data, CLayout, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(row * self.dim.1 + col)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(index)    
    }

}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for BaseMatrixMut<Item, Data, FLayout, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(col * self.dim.0 + row)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(index)    
    }

}

impl<
        Item: Scalar,
        Data: DataContainerMut<Item = Item>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for BaseMatrixMut<Item, Data, CustomLayout, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item {
        self.data.get_unchecked_mut(self.stride.0 * row + self.stride.1 * col)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        let row =   index / self.dim.1;
        let col = index % self.dim.1;

        self.get_unchecked_mut(row, col)
    }

}




// pub struct DynamicMatrix<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
// {
//     data: Vec<Item>,
//     dim: (usize, usize),
//     phantom_layout: PhantomData<L>,
//     phantom_r: PhantomData<RS>,
//     phantom_c: PhantomData<CS>,
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
//     DynamicMatrix<Item, L, RS, CS>
// {
//     /// New dynamic matrix with dimensions (rows, cols)
//     pub fn new(rows: usize, cols: usize) -> Self {
//         DynamicMatrix::<Item, L, RS, CS> {
//             data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
//             dim: (rows, cols),
//             phantom_layout: PhantomData,
//             phantom_r: PhantomData,
//             phantom_c: PhantomData,
//         }
//     }
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> Dimensions
//     for DynamicMatrix<Item, L, RS, CS>
// {
//     fn dim(&self) -> (usize, usize) {
//         self.dim
//     }
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> LayoutType<L>
//     for DynamicMatrix<Item, L, RS, CS>
// {
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> SizeType
//     for DynamicMatrix<Item, L, RS, CS>
// {
//     type R = RS;
//     type C = CS;
// }

// impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
//     SafeRandomAccess for DynamicMatrix<Item, Layout, RS, CS>
// {
//     type Output = Item;

//     #[inline]
//     fn get(&self, row: usize, col: usize) -> Self::Output {
//         *self
//             .data
//             .get(Layout::transform_index(row, col, self.dim()))
//             .unwrap()
//     }
//     #[inline]
//     fn get1d(&self, index: usize) -> Self::Output {
//         *self.data.get(index).unwrap()
//     }
// }

// impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
//     UnsafeRandomAccess for DynamicMatrix<Item, Layout, RS, CS>
// {
//     type Output = Item;

//     #[inline]
//     unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
//         *self
//             .data
//             .get_unchecked(Layout::transform_index(row, col, self.dim()))
//     }
//     #[inline]
//     unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
//         *self.data.get_unchecked(index)
//     }
// }

// impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
//     SafeMutableRandomAccess for DynamicMatrix<Item, Layout, RS, CS>
// {
//     type Output = Item;

//     #[inline]
//     fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
//         let dim = self.dim();
//         self.data
//             .get_mut(Layout::transform_index(row, col, dim))
//             .unwrap()
//     }
//     #[inline]
//     fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.data.get_mut(index).unwrap()
//     }
// }

// impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
//     UnsafeMutableRandomAccess for DynamicMatrix<Item, Layout, RS, CS>
// {
//     type Output = Item;

//     #[inline]
//     unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
//         let dim = self.dim();
//         self.data
//             .get_unchecked_mut(Layout::transform_index(row, col, dim))
//     }
//     #[inline]
//     unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.data.get_unchecked_mut(index)
//     }
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> Pointer
//     for DynamicMatrix<Item, L, RS, CS>
// {
//     type Item = Item;

//     fn as_ptr(&self) -> *const Item {
//         self.data.as_ptr()
//     }
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> PointerMut
//     for DynamicMatrix<Item, L, RS, CS>
// {
//     type Item = Item;

//     fn as_mut_ptr(&mut self) -> *mut Item {
//         self.data.as_mut_ptr()
//     }
// }

// impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> Stride
//     for DynamicMatrix<Item, L, RS, CS>
// {
//     fn row_stride(&self) -> usize {
//         L::default_stride(self.dim()).0
//     }

//     fn column_stride(&self) -> usize {
//         L::default_stride(self.dim()).1
//     }
// }
