//! Slice operations for matrices

use super::Matrix;
use crate::base_matrix::BaseMatrix;
use crate::data_container::{DataContainer, DataContainerMut, SliceContainer, SliceContainerMut};
use crate::layouts::*;
use crate::traits::*;
use crate::types::{IndexType, Scalar};

macro_rules! block_matrix {
    ($Layout:ident, $StrideLayout:ident) => {
        impl<
                'a,
                Item: Scalar,
                Data: DataContainer<Item = Item>,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > Matrix<Item, BaseMatrix<Item, Data, $Layout, RS, CS>, $Layout, RS, CS>
        {
            pub fn block(
                &'a self,
                top_left: (IndexType, IndexType),
                dim: (IndexType, IndexType),
            ) -> Matrix<
                Item,
                BaseMatrix<Item, SliceContainer<'a, Item>, $StrideLayout, RS, CS>,
                $StrideLayout,
                RS,
                CS,
            > {
                assert!(
                    (top_left.0 + dim.0 <= self.layout().dim().0) & (top_left.1 + dim.1 <= self.layout().dim().1),
                    "Lower right corner of block {:?} out of bounds for matrix with dim {:?}",
                    (top_left.0 + dim.0 - 1, top_left.1 + dim.1 - 1),
                    self.layout().dim()
                );
                let start_index = self.layout().convert_2d_raw(
                    top_left.0,
                    top_left.1,
                );
                let end_index = self.layout().convert_2d_raw(
                    top_left.0 + dim.0 - 1,
                    top_left.1 + dim.1 - 1,

                ) + 1;

                let data = SliceContainer::<'a, Item>::new(self.get_slice(start_index, end_index));

                Matrix::<
                    Item,
                    BaseMatrix<Item, SliceContainer<'a, Item>, $StrideLayout, RS, CS>,
                    $StrideLayout,
                    RS,
                    CS,
                >::from_data(data, dim, self.layout().stride())
            }
        }
        impl<
                'a,
                Item: Scalar,
                Data: DataContainerMut<Item = Item>,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
            > Matrix<Item, BaseMatrix<Item, Data, $Layout, RS, CS>, $Layout, RS, CS>
        {
            pub fn block_mut(
                &'a mut self,
                top_left: (IndexType, IndexType),
                dim: (IndexType, IndexType),
            ) -> Matrix<
                Item,
                BaseMatrix<Item, SliceContainerMut<'a, Item>, $StrideLayout, RS, CS>,
                $StrideLayout,
                RS,
                CS,
            > {
                let stride = self.layout().stride();

                assert!(
                    (top_left.0 + dim.0 <= self.layout().dim().0) & (top_left.1 + dim.1 <= self.layout().dim().1),
                    "Lower right corner of block {:?} out of bounds for matrix with dim {:?}",
                    (top_left.0 + dim.0 - 1, top_left.1 + dim.1 - 1),
                    self.layout().dim()
                );

                let start_index = self.layout().convert_2d_raw(
                    top_left.0,
                    top_left.1,

                );
                let end_index = self.layout().convert_2d_raw(
                    top_left.0 + dim.0 - 1,
                    top_left.1 + dim.1 - 1,
                ) + 1;

                let data =
                    SliceContainerMut::<'a, Item>::new(self.get_slice_mut(start_index, end_index));

                Matrix::<
                    Item,
                    BaseMatrix<Item, SliceContainerMut<'a, Item>, $StrideLayout, RS, CS>,
                    $StrideLayout,
                    RS,
                    CS,
                >::from_data(data, dim, stride)
            }
        }
    };
}

block_matrix!(RowMajor, ArbitraryStrideRowMajor);
block_matrix!(ColumnMajor, ArbitraryStrideColumnMajor);
block_matrix!(ArbitraryStrideRowMajor, ArbitraryStrideRowMajor);
block_matrix!(ArbitraryStrideColumnMajor, ArbitraryStrideColumnMajor);



#[cfg(test)]
mod test {

    use super::*;
    use crate::matrix::*;

    #[test]
    fn test_simple_slice() {
        let mut mat = MatrixD::<f64, RowMajor>::from_zeros(3, 4);
        *mat.get_mut(1, 2) = 1.0;

        let slice = mat.block((0, 1), (2, 2));

        assert_eq!(slice.get(1, 1), 1.0);
        assert_eq!(slice.get1d(3), 1.0);
    }

    #[test]
    fn test_double_slice() {
        let mut mat = MatrixD::<f64, RowMajor>::from_zeros(3, 4);
        *mat.get_mut(1, 2) = 1.0;

        let slice1 = mat.block((0, 1), (3, 3));
        let slice2 = slice1.block((1, 0), (2, 2));

        assert_eq!(slice1.get(1, 1), 1.0);
        assert_eq!(slice2.get(0, 1), 1.0);
    }
    

}
