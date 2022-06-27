//! Base matrix from a slice
use crate::traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

struct ContinuousSlice;
struct DiscontinuousSlice;

pub trait SliceType {}

impl SliceType for ContinuousSlice {}
impl SliceType for DiscontinuousSlice {}

pub struct SliceMatrix<
    'a,
    Item: Scalar,
    L: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    ST: SliceType,
> {
    data: &'a [Item],
    start: usize,
    dim: (usize, usize),
    stride: (usize, usize),
    phantom_layout: PhantomData<L>,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
    phantom_st: PhantomData<ST>,
}

pub struct SliceMatrixMut<
    'a,
    Item: Scalar,
    L: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    ST: SliceType,
> {
    data: &'a mut [Item],
    start: usize,
    dim: (usize, usize),
    stride: (usize, usize),
    phantom_layout: PhantomData<L>,
    phantom_r: PhantomData<RS>,
    phantom_c: PhantomData<CS>,
    phantom_st: PhantomData<ST>,
}

impl<
        'a,
        Item: Scalar,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        ST: SliceType,
    > SliceMatrix<'a, Item, L, RS, CS, ST>
{
    /// New slice with dimensions (rows, cols).
    pub fn new(
        slice: &'a [Item],
        start: usize,
        rows: usize,
        cols: usize,
        stride: (usize, usize),
    ) -> Self {
        SliceMatrix {
            data: slice,
            start,
            dim: (rows, cols),
            stride,
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
            phantom_st: PhantomData,
        }
    }

    fn check_dim(&self, row: usize, col: usize) {
        assert!(row < self.dim.0 && col < self.dim.1)
    }

    fn check_dim1d(&self, elem: usize) {
        assert!(elem < self.dim.0 * self.dim.1)
    }
}

impl<
        'a,
        Item: Scalar,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        ST: SliceType,
    > SliceMatrixMut<'a, Item, L, RS, CS, ST>
{
    /// New mutable slice with dimensions (rows, cols).
    pub fn new(
        slice: &'a mut [Item],
        start: usize,
        rows: usize,
        cols: usize,
        stride: (usize, usize),
    ) -> Self {
        SliceMatrixMut {
            data: slice,
            start,
            dim: (rows, cols),
            stride,
            phantom_layout: PhantomData,
            phantom_r: PhantomData,
            phantom_c: PhantomData,
            phantom_st: PhantomData,
        }
    }

    fn check_dim(&self, row: usize, col: usize) {
        assert!(row < self.dim.0 && col < self.dim.1)
    }

    fn check_dim1d(&self, elem: usize) {
        assert!(elem < self.dim.0 * self.dim.1)
    }
}

macro_rules! slice_matrix_traits {
    ($SliceType:ident) => {
        impl<
                'a,
                Item: Scalar,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
                ST: SliceType,
            > Dimensions for $SliceType<'a, Item, L, RS, CS, ST>
        {
            fn dim(&self) -> (usize, usize) {
                self.dim
            }
        }

        impl<
                'a,
                Item: Scalar,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
                ST: SliceType,
            > LayoutType<L> for $SliceType<'a, Item, L, RS, CS, ST>
        {
        }

        impl<
                'a,
                Item: Scalar,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
                ST: SliceType,
            > SizeType for $SliceType<'a, Item, L, RS, CS, ST>
        {
            type R = RS;
            type C = CS;
        }

        impl<
                'a,
                Item: Scalar,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
                ST: SliceType,
            > Stride for $SliceType<'a, Item, L, RS, CS, ST>
        {
            fn row_stride(&self) -> usize {
                self.stride.0
            }

            fn column_stride(&self) -> usize {
                self.stride.1
            }
        }

        impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
            SafeRandomAccess for $SliceType<'a, Item, L, RS, CS, ContinuousSlice>
        {
            type Output = Item;

            #[inline]
            fn get(&self, row: usize, col: usize) -> Self::Output {
                self.check_dim(row, col);
                let index = self.start + row * self.stride.0 + col * self.stride.1;
                *self.data.get(index).unwrap()
            }

            #[inline]
            fn get1d(&self, elem: usize) -> Self::Output {
                self.check_dim1d(elem);
                *self.data.get(self.start + elem).unwrap()
            }
        }

        impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> SafeRandomAccess
            for $SliceType<'a, Item, CLayout, RS, CS, DiscontinuousSlice>
        {
            type Output = Item;

            #[inline]
            fn get(&self, row: usize, col: usize) -> Self::Output {
                self.check_dim(row, col);
                let index = self.start + row * self.stride.0 + col * self.stride.1;
                *self.data.get(index).unwrap()
            }

            #[inline]
            fn get1d(&self, elem: usize) -> Self::Output {
                self.check_dim1d(elem);
                let row = elem / self.dim.1;
                let col = elem & self.dim.1;
                self.get(row, col)
            }
        }

        impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> SafeRandomAccess
            for $SliceType<'a, Item, FLayout, RS, CS, DiscontinuousSlice>
        {
            type Output = Item;

            #[inline]
            fn get(&self, row: usize, col: usize) -> Self::Output {
                self.check_dim(row, col);
                let index = self.start + row * self.stride.0 + col * self.stride.1;
                *self.data.get(index).unwrap()
            }

            #[inline]
            fn get1d(&self, elem: usize) -> Self::Output {
                self.check_dim1d(elem);
                let row = elem & self.dim.0;
                let col = elem / self.dim.0;
                self.get(row, col)
            }
        }

        impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
            UnsafeRandomAccess for $SliceType<'a, Item, L, RS, CS, ContinuousSlice>
        {
            type Output = Item;

            #[inline]
            unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
                let index = self.start + row * self.stride.0 + col * self.stride.1;
                *self.data.get_unchecked(index)
            }

            #[inline]
            unsafe fn get1d_unchecked(&self, elem: usize) -> Self::Output {
                *self.data.get_unchecked(self.start + elem)
            }
        }

        impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> UnsafeRandomAccess
            for $SliceType<'a, Item, CLayout, RS, CS, DiscontinuousSlice>
        {
            type Output = Item;

            #[inline]
            unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
                let index = self.start + row * self.stride.0 + col * self.stride.1;
                *self.data.get_unchecked(index)
            }

            #[inline]
            unsafe fn get1d_unchecked(&self, elem: usize) -> Self::Output {
                let row = elem / self.dim.1;
                let col = elem & self.dim.1;
                self.get_unchecked(row, col)
            }
        }

        impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> UnsafeRandomAccess
            for $SliceType<'a, Item, FLayout, RS, CS, DiscontinuousSlice>
        {
            type Output = Item;

            #[inline]
            unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
                let index = self.start + row * self.stride.0 + col * self.stride.1;
                *self.data.get_unchecked(index)
            }

            #[inline]
            unsafe fn get1d_unchecked(&self, elem: usize) -> Self::Output {
                let row = elem & self.dim.0;
                let col = elem / self.dim.0;
                self.get_unchecked(row, col)
            }
        }
    };
}

impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
    SafeMutableRandomAccess for SliceMatrixMut<'a, Item, L, RS, CS, ContinuousSlice>
{
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.check_dim(row, col);
        let index = self.start + row * self.stride.0 + col * self.stride.1;
        self.data.get_mut(index).unwrap()
    }

    #[inline]
    fn get1d_mut(&mut self, elem: usize) -> &mut Self::Output {
        self.check_dim1d(elem);
        self.data.get_mut(self.start + elem).unwrap()
    }
}

impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> SafeMutableRandomAccess
    for SliceMatrixMut<'a, Item, CLayout, RS, CS, DiscontinuousSlice>
{
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.check_dim(row, col);
        let index = self.start + row * self.stride.0 + col * self.stride.1;
        self.data.get_mut(index).unwrap()
    }

    #[inline]
    fn get1d_mut(&mut self, elem: usize) -> &mut Self::Output {
        self.check_dim1d(elem);
        let row = elem / self.dim.1;
        let col = elem & self.dim.1;
        self.get_mut(row, col)
    }
}

impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> SafeMutableRandomAccess
    for SliceMatrixMut<'a, Item, FLayout, RS, CS, DiscontinuousSlice>
{
    type Output = Item;

    #[inline]
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        self.check_dim(row, col);
        let index = self.start + row * self.stride.0 + col * self.stride.1;
        self.data.get_mut(index).unwrap()
    }

    #[inline]
    fn get1d_mut(&mut self, elem: usize) -> &mut Self::Output {
        self.check_dim1d(elem);
        let row = elem & self.dim.0;
        let col = elem / self.dim.0;
        self.get_mut(row, col)
    }
}

impl<'a, Item: Scalar, L: LayoutIdentifier, RS: SizeIdentifier, CS: SizeIdentifier>
    UnsafeMutableRandomAccess for SliceMatrixMut<'a, Item, L, RS, CS, ContinuousSlice>
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let index = self.start + row * self.stride.0 + col * self.stride.1;
        self.data.get_unchecked_mut(index)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, elem: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(self.start + elem)
    }
}

impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> UnsafeMutableRandomAccess
    for SliceMatrixMut<'a, Item, CLayout, RS, CS, DiscontinuousSlice>
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let index = self.start + row * self.stride.0 + col * self.stride.1;
        self.data.get_unchecked_mut(index)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, elem: usize) -> &mut Self::Output {
        let row = elem / self.dim.1;
        let col = elem & self.dim.1;
        self.get_unchecked_mut(row, col)
    }
}

impl<'a, Item: Scalar, RS: SizeIdentifier, CS: SizeIdentifier> UnsafeMutableRandomAccess
    for SliceMatrixMut<'a, Item, FLayout, RS, CS, DiscontinuousSlice>
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output {
        let index = self.start + row * self.stride.0 + col * self.stride.1;
        self.data.get_unchecked_mut(index)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, elem: usize) -> &mut Self::Output {
        let row = elem & self.dim.0;
        let col = elem / self.dim.0;
        self.get_unchecked_mut(row, col)
    }
}

slice_matrix_traits!(SliceMatrix);
slice_matrix_traits!(SliceMatrixMut);