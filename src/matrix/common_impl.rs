//! Implementation of matrix traits and methods

use crate::matrix::{Matrix, MatrixMut};
use crate::traits::*;
use crate::types::{IndexType, Scalar};

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > LayoutType<L> for Matrix<Item, MatImpl, L, RS, CS>
{
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > SizeType for Matrix<Item, MatImpl, L, RS, CS>
{
    type R = RS;
    type C = CS;
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Dimensions for Matrix<Item, MatImpl, L, RS, CS>
{
    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        self.0.dim()
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.0.number_of_elements()
    }
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Stride for Matrix<Item, MatImpl, L, RS, CS>
{
    #[inline]
    fn row_stride(&self) -> IndexType {
        self.0.row_stride()
    }

    #[inline]
    fn column_stride(&self) -> IndexType {
        self.0.column_stride()
    }
}

impl<
        Item: Scalar,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccess for Matrix<Item, MatImpl, L, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
        self.0.get_unchecked(row, col)
    }

    #[inline]
    unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
        self.0.get1d_unchecked(index)
    }
}

impl<
        Item: Scalar,
        MatImpl: MatrixTraitMut<Item, L, RS, CS>,
        L: LayoutIdentifier,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessMut for Matrix<Item, MatImpl, L, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, row: IndexType, col: IndexType) -> &mut Self::Item {
        self.0.get_unchecked_mut(row, col)
    }

    #[inline]
    unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
        self.0.get1d_unchecked_mut(index)
    }
}



// // Common implementantions for mutable and non-mutable matrices
// macro_rules! common_impl {
//     ($MatrixType:ident, $MatTrait:ident, $RS:ident, $CS:ident) => {
//         impl<Item: Scalar, L: LayoutIdentifier, MatImpl: $MatTrait<Item, L, $RS, $CS>> LayoutType<L>
//             for $MatrixType<Item, MatImpl, L, $RS, $CS>
//         {
//         }

//         impl<Item: Scalar, L: LayoutIdentifier, MatImpl: $MatTrait<Item, L, $RS, $CS>> SizeType
//             for $MatrixType<Item, MatImpl, L, $RS, $CS>
//         {
//             type R = $RS;
//             type C = $CS;
//         }

//         impl<Item: Scalar, L: LayoutIdentifier, MatImpl: $MatTrait<Item, L, $RS, $CS>> Dimensions
//             for $MatrixType<Item, MatImpl, L, $RS, $CS>
//         {
//             #[inline]
//             fn dim(&self) -> (IndexType, IndexType) {
//                 self.0.dim()
//             }

//             #[inline]
//             fn number_of_elements(&self) -> IndexType {
//                 self.0.number_of_elements()
//             }
//         }

//         impl<Item: Scalar, L: LayoutIdentifier, MatImpl: $MatTrait<Item, L, $RS, $CS>> Stride
//             for $MatrixType<Item, MatImpl, L, $RS, $CS>
//         {
//             #[inline]
//             fn row_stride(&self) -> IndexType {
//                 self.0.row_stride()
//             }

//             #[inline]
//             fn column_stride(&self) -> IndexType {
//                 self.0.column_stride()
//             }
//         }

//         impl<Item: Scalar, L: LayoutIdentifier, MatImpl: $MatTrait<Item, L, $RS, $CS>>
//             UnsafeRandomAccess for $MatrixType<Item, MatImpl, L, $RS, $CS>
//         {
//             type Item = Item;

//             #[inline]
//             unsafe fn get_unchecked(&self, row: IndexType, col: IndexType) -> Self::Item {
//                 self.0.get_unchecked(row, col)
//             }

//             #[inline]
//             unsafe fn get1d_unchecked(&self, index: IndexType) -> Self::Item {
//                 self.0.get1d_unchecked(index)
//             }
//         }
//     };
// }

// // Implementations specific to mutable types.
// macro_rules! mutable_impl {
//     ($MatrixType:ident, $MatTrait:ident, $RS:ident, $CS:ident) => {
//         impl<Item: Scalar, L: LayoutIdentifier, MatImpl: $MatTrait<Item, L, $RS, $CS>>
//             UnsafeRandomAccessMut for $MatrixType<Item, MatImpl, L, $RS, $CS>
//         {
//             type Item = Item;

//             #[inline]
//             unsafe fn get_unchecked_mut(
//                 &mut self,
//                 row: IndexType,
//                 col: IndexType,
//             ) -> &mut Self::Item {
//                 self.0.get_unchecked_mut(row, col)
//             }

//             #[inline]
//             unsafe fn get1d_unchecked_mut(&mut self, index: IndexType) -> &mut Self::Item {
//                 self.0.get1d_unchecked_mut(index)
//             }
//         }
//     };
// }

// // Implementations specific to mutable types.
// macro_rules! eval_impl {
//     ($MatrixType:ident, $MatTrait:ident, $RS:ident, $CS:ident) => {
//         impl<Item: Scalar, MatImpl: $MatTrait<Item, CLayout, $RS, $CS>>
//             $MatrixType<Item, MatImpl, CLayout, $RS, $CS>
//         {
//         }
//     };
// }

// macro_rules! methods {
//     ($MatrixType:ident, $MatTrait:ident, $Macro:ident) => {
//         $Macro!($MatrixType, $MatTrait, Dynamic, Fixed1);
//         $Macro!($MatrixType, $MatTrait, Fixed1, Dynamic);
//         $Macro!($MatrixType, $MatTrait, Dynamic, Dynamic);

//         $Macro!($MatrixType, $MatTrait, Fixed2, Fixed2);
//         $Macro!($MatrixType, $MatTrait, Fixed2, Fixed1);
//         $Macro!($MatrixType, $MatTrait, Fixed1, Fixed2);

//         $Macro!($MatrixType, $MatTrait, Fixed3, Fixed3);
//         $Macro!($MatrixType, $MatTrait, Fixed3, Fixed1);
//         $Macro!($MatrixType, $MatTrait, Fixed1, Fixed3);
//     };
// }

// methods!(Matrix, MatrixTrait, common_impl);
// methods!(MatrixMut, MatrixTraitMut, common_impl);
// methods!(MatrixMut, MatrixTraitMut, mutable_impl);
// methods!(Matrix, MatrixTrait, eval_impl);
// methods!(MatrixMut, MatrixTraitMut, eval_impl);
