//! Slice operations for matrices

use super::{GenericBaseMatrix, GenericBaseMatrixMut, Matrix, SliceMatrix, SliceMatrixMut};
use crate::base_matrix::BaseMatrix;
use crate::data_container::{DataContainer, DataContainerMut, SliceContainer, SliceContainerMut};
use crate::layouts::*;
use crate::traits::*;
use crate::types::{IndexType, Scalar};

macro_rules! block_matrix {
    ($Layout:ident, $StrideLayout:ident, $RS:ident, $CS:ident) => {
        impl<Item: Scalar, Data: DataContainer<Item = Item>>
            Matrix<Item, BaseMatrix<Item, Data, $Layout, $RS, $CS>, $Layout, $RS, $CS>
        {
            pub fn block<'a>(
                &'a self,
                top_left: (IndexType, IndexType),
                dim: (IndexType, IndexType),
            ) -> SliceMatrix<'a, Item, $StrideLayout, $RS, $CS> {
                assert!(
                    (top_left.0 + dim.0 <= self.layout().dim().0)
                        & (top_left.1 + dim.1 <= self.layout().dim().1),
                    "Lower right corner {:?} out of bounds for matrix with dim {:?}",
                    (top_left.0 + dim.0 - 1, top_left.1 + dim.1 - 1),
                    self.layout().dim()
                );
                let start_index = self.layout().convert_2d_raw(top_left.0, top_left.1);
                unsafe {
                    SliceMatrix::<'a, Item, $StrideLayout, $RS, $CS>::from_pointer(
                        self.get_pointer().offset(start_index as isize),
                        dim,
                        self.layout().stride(),
                    )
                }
            }
        }
        impl<Item: Scalar, Data: DataContainerMut<Item = Item>>
            Matrix<Item, BaseMatrix<Item, Data, $Layout, $RS, $CS>, $Layout, $RS, $CS>
        {
            pub fn block_mut<'a>(
                &'a mut self,
                top_left: (IndexType, IndexType),
                dim: (IndexType, IndexType),
            ) -> SliceMatrixMut<'a, Item, $StrideLayout, $RS, $CS> {
                assert!(
                    (top_left.0 + dim.0 <= self.layout().dim().0)
                        & (top_left.1 + dim.1 <= self.layout().dim().1),
                    "Lower right corner {:?} out of bounds for matrix with dim {:?}",
                    (top_left.0 + dim.0 - 1, top_left.1 + dim.1 - 1),
                    self.layout().dim()
                );
                let start_index = self.layout().convert_2d_raw(top_left.0, top_left.1);

                unsafe {
                    SliceMatrixMut::<'a, Item, $StrideLayout, $RS, $CS>::from_pointer(
                        self.get_pointer_mut().offset(start_index as isize),
                        dim,
                        self.layout().stride(),
                    )
                }
            }
        }
    };
}

macro_rules! subdivide_matrix {
    ($Layout:ident, $StrideLayout:ident) => {
        impl<Item: Scalar, Data: DataContainerMut<Item = Item>>
            GenericBaseMatrixMut<Item, $Layout, Data, Dynamic, Dynamic>
        {
            pub fn split_in_four<'a>(
                &'a mut self,
                split_at: (usize, usize),
            ) -> (
                SliceMatrixMut<'a, Item, $StrideLayout, Dynamic, Dynamic>,
                SliceMatrixMut<'a, Item, $StrideLayout, Dynamic, Dynamic>,
                SliceMatrixMut<'a, Item, $StrideLayout, Dynamic, Dynamic>,
                SliceMatrixMut<'a, Item, $StrideLayout, Dynamic, Dynamic>,
            ) {
                let dim = self.layout().dim();
                let stride = self.layout().stride();
                let ptr = self.get_pointer_mut();
                let dim0 = split_at;
                let dim1 = (split_at.0, dim.1 - split_at.1);
                let dim2 = (dim.0 - split_at.0, split_at.1);
                let dim3 = (dim.0 - split_at.0, dim.1 - split_at.1);

                let origin0 = (0, 0);
                let origin1 = (0, split_at.1);
                let origin2 = (split_at.0, 0);
                let origin3 = split_at;

                let start0 = self.layout().convert_2d_raw(origin0.0, origin0.1) as isize;
                let start1 = self.layout().convert_2d_raw(origin1.0, origin1.1) as isize;
                let start2 = self.layout().convert_2d_raw(origin2.0, origin2.1) as isize;
                let start3 = self.layout().convert_2d_raw(origin3.0, origin3.1) as isize;

                let blocks = unsafe {
                    (
                        SliceMatrixMut::<'a, Item, $StrideLayout, Dynamic, Dynamic>::from_pointer(
                            ptr.offset(start0),
                            dim0,
                            stride,
                        ),
                        SliceMatrixMut::<'a, Item, $StrideLayout, Dynamic, Dynamic>::from_pointer(
                            ptr.offset(start1),
                            dim1,
                            stride,
                        ),
                        SliceMatrixMut::<'a, Item, $StrideLayout, Dynamic, Dynamic>::from_pointer(
                            ptr.offset(start2),
                            dim2,
                            stride,
                        ),
                        SliceMatrixMut::<'a, Item, $StrideLayout, Dynamic, Dynamic>::from_pointer(
                            ptr.offset(start3),
                            dim3,
                            stride,
                        ),
                    )
                };
                blocks
            }
        }
    };
}

block_matrix!(RowMajor, ArbitraryStrideRowMajor, Dynamic, Dynamic);
block_matrix!(ColumnMajor, ArbitraryStrideColumnMajor, Dynamic, Dynamic);
block_matrix!(
    ArbitraryStrideRowMajor,
    ArbitraryStrideRowMajor,
    Dynamic,
    Dynamic
);
block_matrix!(
    ArbitraryStrideColumnMajor,
    ArbitraryStrideColumnMajor,
    Dynamic,
    Dynamic
);

subdivide_matrix!(RowMajor, ArbitraryStrideRowMajor);

#[cfg(test)]
mod test {

    use super::*;
    use crate::matrix::*;

    #[test]
    fn test_simple_slice() {
        let mut mat = MatrixD::<f64, RowMajor>::zeros_from_dim(3, 4);
        *mat.get_mut(1, 2) = 1.0;

        let slice = mat.block((0, 1), (2, 2));

        assert_eq!(slice.get(1, 1), 1.0);
        assert_eq!(slice.get1d(3), 1.0);
    }

    #[test]
    fn test_double_slice() {
        let mut mat = MatrixD::<f64, RowMajor>::zeros_from_dim(3, 4);
        *mat.get_mut(1, 2) = 1.0;

        let slice1 = mat.block((0, 1), (3, 3));
        let slice2 = slice1.block((1, 0), (2, 2));

        assert_eq!(slice1.get(1, 1), 1.0);
        assert_eq!(slice2.get(0, 1), 1.0);
    }

    #[test]
    fn test_disjoint_slices() {
        let mut mat = MatrixD::<f64, RowMajor>::zeros_from_dim(4, 5);
        *mat.get_mut(1, 1) = 1.0;
        *mat.get_mut(0, 2) = 2.0;

        let (mut slice1, mut slice2, mut slice3, mut slice4) = mat.split_in_four((2, 3));

        *slice1.get_mut(0, 0) = 2.0;
        *slice2.get_mut(0, 0) = 3.0;

        // unsafe {
        //     let ptr1 = mat.get_pointer_mut();
        //     let ptr2 = mat.get_pointer_mut();

        //     let slice1 = std::slice::from_raw_parts_mut(ptr1, 2);
        //     let slice2 = std::slice::from_raw_parts_mut(ptr2, 3);

        //     // let slice1 = mat.block_mut((0, 0), (3, 2));
        //     // let slice2 = mat.block_mut((0, 2), (3, 2));

        //     *slice1.get_mut(0).unwrap() = 1.0;
        //     *slice2.get_mut(1).unwrap() = 2.0;

        //     assert_eq!(*slice1.get_mut(0).unwrap(), 1.0 as f64);
        //     assert_eq!(*slice2.get_mut(1).unwrap(), 2.0 as f64);
        // }
    }
}
