//! The base matrix data types
use crate::matrix_traits::*;
use crate::iterators::*;
use cauchy::Scalar;

/// Base matrix with C Layout
pub struct DynamicMatrixCLayout<Item: Scalar> {
    data: Vec<Item>,
    dim: (usize, usize),
}



/// Base matrix with Fortran Layout
pub struct DynamicMatrixFortranLayout<Item: Scalar> {
    data: Vec<Item>,
    dim: (usize, usize),
}

impl<Item: Scalar> DynamicMatrixCLayout<Item> {
    // New C Layout base matrix with dimensions (rows, cols)
    pub fn new(rows: usize, cols: usize) -> Self {
        DynamicMatrixCLayout::<Item> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
            dim: (rows, cols),
        }
    }
}

impl<Item: Scalar> DynamicMatrixFortranLayout<Item> {
    // New Fortran Layout base matrix with dimensions (rows, cols)
    pub fn new(rows: usize, cols: usize) -> Self {
        DynamicMatrixFortranLayout::<Item> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
            dim: (rows, cols),
        }
    }
}

impl<Item: Scalar> Dimensions for DynamicMatrixCLayout<Item> {
    fn dim(&self) -> (usize, usize) {
        self.dim
    }
}

impl<Item: Scalar> Dimensions for DynamicMatrixFortranLayout<Item> {
    fn dim(&self) -> (usize, usize) {
        self.dim
    }
}

impl<Item: Scalar> LayoutType for DynamicMatrixCLayout<Item> {
    type L = CLayout;

}

impl<Item: Scalar> LayoutType for DynamicMatrixFortranLayout<Item> {
    type L = FortranLayout;

}

impl<Item: Scalar> SizeType for DynamicMatrixCLayout<Item> {
    type S = MatrixD;

}

impl<Item: Scalar> SizeType for DynamicMatrixFortranLayout<Item> {
    type S = MatrixD;

}

impl<'a, Item: Scalar> Iterable<'a, Item, CopiedSliceIterator<'a, Item>>
    for DynamicMatrixCLayout<Item>
{
    fn iter(&'a self) -> CopiedSliceIterator<'a, Item> {
        self.data.iter().copied()
    }
}

impl<'a, Item: Scalar> Iterable<'a, Item, CopiedSliceIterator<'a, Item>>
    for DynamicMatrixFortranLayout<Item>
{
    fn iter(&'a self) -> CopiedSliceIterator<'a, Item> {
        self.data.iter().copied()
    }
}

impl<'a, Item: Scalar> IterableMut<'a, Item, SliceIteratorMut<'a, Item>>
    for DynamicMatrixCLayout<Item>
{
    fn iter_mut(&'a mut self) -> SliceIteratorMut<'a, Item> {
        self.data.iter_mut()
    }
}

impl<'a, Item: Scalar> IterableMut<'a, Item, SliceIteratorMut<'a, Item>>
    for DynamicMatrixFortranLayout<Item>
{
    fn iter_mut(&'a mut self) -> SliceIteratorMut<'a, Item> {
        self.data.iter_mut()
    }
}


impl<Item: Scalar> SafeRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.data.get(row * self.dim.1 + col).unwrap().clone()
    }
}

impl<Item: Scalar> UnsafeRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.data.get_unchecked(row * self.dim.1 + col).clone()
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_mut(row * self.dim.1 + col).unwrap()
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(row * self.dim.1 + col)
    }
}

impl<Item: Scalar> UnsafeRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.data.get_unchecked(col * self.dim.0 + row).clone()
    }
}

impl<Item: Scalar> SafeRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.data.get(col * self.dim.0 + row).unwrap().clone()
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(col * self.dim.0 + row)
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_mut(col * self.dim.0 + row).unwrap()
    }
}
