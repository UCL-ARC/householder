//! Base matrix from a slice
use crate::traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub struct SliceMatrix<
    'a,
    Item: Scalar,
    L: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
> {
    data: &'a [Item],
    dim: (usize, usize),
    phantom_layout: PhantomData<L>,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
}

pub struct SliceMatrixMut<
    'a,
    Item: Scalar,
    L: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
> {
    data: &'a mut [Item],
    dim: (usize, usize),
    phantom_layout: PhantomData<L>,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
}

impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
    SliceMatrix<'a, Item, L, RS, CS>
{
    /// New slice matrix with dimensions (rows, cols)
    ///
    /// The product of row and col must be identical to the
    /// length of the slice.
    pub fn new(slice: &'a [Item], rows: usize, cols: usize) -> Self {
        assert_eq!(
            rows * cols,
            slice.len(),
            "rows x cols ({} x {}) must equal slice.len ({})",
            rows,
            cols,
            slice.len()
        );
        SliceMatrix {
            data: slice,
            dim: (rows, cols),
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
    SliceMatrixMut<'a, Item, L, RS, CS>
{
    /// New mutable slice matrix with dimensions (rows, cols)
    ///
    /// The product of row and col must be identical to the
    /// length of the slice.
    pub fn new(slice: &'a mut [Item], rows: usize, cols: usize) -> Self {
        assert_eq!(
            rows * cols,
            slice.len(),
            "rows x cols ({} x {}) must equal slice.len ({})",
            rows,
            cols,
            slice.len()
        );

        SliceMatrixMut {
            data: slice,
            dim: (rows, cols),
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
        }
    }
}

macro_rules! slice_matrix_impl {
    ($SliceMatType:ident) => {
        impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
            Dimensions for $SliceMatType<'a, Item, L, RS, CS>
        {
            fn dim(&self) -> (usize, usize) {
                self.dim
            }
        }

        impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
            LayoutType<L> for $SliceMatType<'a, Item, L, RS, CS>
        {
        }

        impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> SizeType
            for $SliceMatType<'a, Item, L, RS, CS>
        {
            type R = RS;
            type C = CS;
        }

        impl<
                'a,
                Item: Scalar,
                Layout: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > SafeRandomAccess for $SliceMatType<'a, Item, Layout, RS, CS>
        {
            type Output = Item;

            #[inline]
            fn get(&self, row: usize, col: usize) -> Self::Output {
                *self
                    .data
                    .get(Layout::transform_index(row, col, self.dim()))
                    .unwrap()
            }
            #[inline]
            fn get1d(&self, index: usize) -> Self::Output {
                *self.data.get(index).unwrap()
            }
        }

        impl<
                'a,
                Item: Scalar,
                Layout: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > UnsafeRandomAccess for $SliceMatType<'a, Item, Layout, RS, CS>
        {
            type Output = Item;

            #[inline]
            unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
                *self
                    .data
                    .get_unchecked(Layout::transform_index(row, col, self.dim()))
            }
            #[inline]
            unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
                *self.data.get_unchecked(index)
            }
        }

        impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> Pointer
            for $SliceMatType<'a, Item, L, RS, CS>
        {
            type Item = Item;

            fn as_ptr(&self) -> *const Item {
                self.data.as_ptr()
            }
        }
    };
}

slice_matrix_impl!(SliceMatrix);
slice_matrix_impl!(SliceMatrixMut);

impl<'a, Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
    SafeMutableRandomAccess for SliceMatrixMut<'a, Item, Layout, RS, CS>
{
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let dim = self.dim();
        self.data
            .get_mut(Layout::transform_index(row, col, dim))
            .unwrap()
    }
    #[inline]
    fn get1d_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_mut(index).unwrap()
    }
}

impl<'a, Item: Scalar, Layout: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
    UnsafeMutableRandomAccess for SliceMatrixMut<'a, Item, Layout, RS, CS>
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let dim = self.dim();
        self.data
            .get_unchecked_mut(Layout::transform_index(row, col, dim))
    }
    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(index)
    }
}

impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier> PointerMut
    for SliceMatrixMut<'a, Item, L, RS, CS>
{
    type Item = Item;

    fn as_mut_ptr(&mut self) -> *mut Item {
        self.data.as_mut_ptr()
    }
}
