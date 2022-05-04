//! The base matrix data types
use crate::matrix_traits::*;
use std::marker::PhantomData;
use cauchy::Scalar;

pub struct DynamicMatrix<Item: Scalar, L: LayoutIdentifier> {
    data: Vec<Item>,
    dim: (usize, usize),
    phantom_layout: PhantomData<L>
}

impl<Item: Scalar, L: LayoutIdentifier> DynamicMatrix<Item, L> {
    // New matrix with dimensions (rows, cols)
    pub fn new(rows: usize, cols: usize) -> Self {
        DynamicMatrix::<Item, L> {
            data: vec![num::cast::<f64, Item>(0.0).unwrap(); rows * cols],
            dim: (rows, cols),
            phantom_layout: PhantomData
        }
    }
}

impl<Item: Scalar, L: LayoutIdentifier> Dimensions for DynamicMatrix<Item, L> {
    fn dim(&self) -> (usize, usize) {
        self.dim
    }
}


impl<Item: Scalar, L: LayoutIdentifier> LayoutType for DynamicMatrix<Item, L> {
    type L = L;
}


impl<Item: Scalar, L: LayoutIdentifier> SizeType for DynamicMatrix<Item, L> {
    type S = MatrixD;
}


impl<Item: Scalar, Layout: LayoutIdentifier> SafeRandomAccess for DynamicMatrix<Item, Layout> {
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

impl<Item: Scalar, Layout: LayoutIdentifier> UnsafeRandomAccess for DynamicMatrix<Item, Layout> {
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

impl<Item: Scalar, Layout: LayoutIdentifier> SafeMutableRandomAccess for DynamicMatrix<Item, Layout> {
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

impl<Item: Scalar, Layout: LayoutIdentifier> UnsafeMutableRandomAccess for DynamicMatrix<Item, Layout> {
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

impl<Item: Scalar, L: LayoutIdentifier> Pointer for DynamicMatrix<Item, L> {
    type Item = Item;

    fn as_ptr(&self) -> *const Item {
        self.data.as_ptr()
    }
}


impl<Item: Scalar, L: LayoutIdentifier> PointerMut for DynamicMatrix<Item, L> {
    type Item = Item;

    fn as_mut_ptr(&mut self) -> *mut Item {
        self.data.as_mut_ptr()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::mat;

    #[test]
    fn test_fortran_ordering() {

        let mut mat = mat![f64, (2, 4), FLayout];

        *mat.get_mut(1, 2) = 3.0;

        assert_eq!(mat.get1d(5), 3.0);
    }

    #[test]
    fn test_c_ordering() {

        let mut mat = mat![f64, (2, 4), CLayout];

        *mat.get_mut(1, 2) = 3.0;

        assert_eq!(mat.get1d(6), 3.0);
    }

}