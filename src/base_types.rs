//! The base matrix data types
use crate::traits::*;
use std::marker::PhantomData;
use cauchy::Scalar;


pub struct DynamicMatrix<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> {
    data: Vec<Item>,
    dim: (usize, usize),
    phantom_layout: PhantomData<L>,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
}

impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> DynamicMatrix<Item, L, RS, CS> {
    /// New dynamic matrix with dimensions (rows, cols)
    pub fn new(rows: usize, cols: usize) -> Self {
        DynamicMatrix::<Item, L, RS, CS> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
            dim: (rows, cols),
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> Dimensions for DynamicMatrix<Item, L, RS, CS> {
    fn dim(&self) -> (usize, usize) {
        self.dim
    }
}

impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> LayoutType<L> for DynamicMatrix<Item, L, RS, CS> {}


impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> SizeType for DynamicMatrix<Item, L, RS, CS> {
    type R = RS;
    type C = CS;
}



impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> SafeRandomAccess for DynamicMatrix<Item, Layout, RS, CS> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        *self.data.get(Layout::transform_index(row, col, self.dim())).unwrap()
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        *self.data.get(index).unwrap()
    }
}

impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> UnsafeRandomAccess for DynamicMatrix<Item, Layout, RS, CS> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        *self.data.get_unchecked(Layout::transform_index(row, col, self.dim()))
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        *self.data.get_unchecked(index)
    }
}

impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> SafeMutableRandomAccess for DynamicMatrix<Item, Layout, RS, CS> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let dim = self.dim();
        self.data.get_mut(Layout::transform_index(row, col, dim)).unwrap()
    }
    #[inline]
    fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_mut(index).unwrap()
    }
}

impl<Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> UnsafeMutableRandomAccess for DynamicMatrix<Item, Layout, RS, CS> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let dim = self.dim();
        self.data.get_unchecked_mut(Layout::transform_index(row, col, dim))
    }
    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(index)
    }
}

impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> Pointer for DynamicMatrix<Item, L, RS, CS> {
    type Item = Item;

    fn as_ptr(&self) -> *const Item {
        self.data.as_ptr()
    }
}


impl<Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> PointerMut for DynamicMatrix<Item, L, RS, CS> {
    type Item = Item;

    fn as_mut_ptr(&mut self) -> *mut Item {
        self.data.as_mut_ptr()
    }
}

