//! The base matrix data types
use crate::matrix_traits::*;
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

impl<Item: Scalar> SafeRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        *self.data.get(row * self.dim.1 + col).unwrap()
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        *self.data.get(index).unwrap()
    }

}

impl<Item: Scalar> UnsafeRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        *self.data.get_unchecked(row * self.dim.1 + col)
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        *self.data.get_unchecked(index)
    }
}

impl<Item: Scalar> SafeMutableRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_mut(row * self.dim.1 + col).unwrap()
    }
    #[inline]
    fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_mut(index).unwrap()
    }
}

impl<Item: Scalar> UnsafeMutableRandomAccess for DynamicMatrixCLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(row * self.dim.1 + col)
    }
    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(index)
    }
}

impl<Item: Scalar> UnsafeRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.data.get_unchecked(col * self.dim.0 + row).clone()
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        *self.data.get_unchecked(index)
    }

}

impl<Item: Scalar> SafeRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        *self.data.get(col * self.dim.0 + row).unwrap()
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        *self.data.get(index).unwrap()
    }

}

impl<Item: Scalar> UnsafeMutableRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(col * self.dim.0 + row)
    }
    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(index)
    }

}

impl<Item: Scalar> SafeMutableRandomAccess for DynamicMatrixFortranLayout<Item> {
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.data.get_mut(col * self.dim.0 + row).unwrap()
    }
    #[inline]
    fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_mut(index).unwrap()
    }
}
