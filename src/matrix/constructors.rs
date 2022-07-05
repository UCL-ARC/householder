//! A collection of routines to construct matrix objects from scratch or existing data.

use crate::base_matrix_mut::BaseMatrixMut;
use crate::data_container::{ArrayContainer, VectorContainer};
use crate::matrix::{MatrixMut};
use crate::traits::*;
use crate::types::{IndexType, Scalar};

// Construct mutable zero matrices

macro_rules! from_zeros_fixed {
    ($RS:ident, $CS:ident, $L:ident) => {
        impl<Item: Scalar>
            MatrixMut<
                Item,
                BaseMatrixMut<Item, ArrayContainer<Item, { $RS::N * $CS::N }>, $L, $RS, $CS>,
                $L,
                $RS,
                $CS,
            >
        {
            pub fn from_zeros() -> Self {
                Self::from_data(
                    ArrayContainer::<Item, { $RS::N * $CS::N }>::new(),
                    ($RS::N, $CS::N),
                )
            }
        }
    };
}

from_zeros_fixed!(Fixed2, Fixed2, CLayout);
from_zeros_fixed!(Fixed1, Fixed2, CLayout);

from_zeros_fixed!(Fixed3, Fixed3, CLayout);
from_zeros_fixed!(Fixed1, Fixed3, CLayout);

from_zeros_fixed!(Fixed2, Fixed2, FLayout);
from_zeros_fixed!(Fixed1, Fixed2, FLayout);

from_zeros_fixed!(Fixed3, Fixed3, FLayout);
from_zeros_fixed!(Fixed1, Fixed3, FLayout);

macro_rules! from_zeros_dynamic_matrix {
    ($L:ident) => {
        impl<Item: Scalar>
            MatrixMut<
                Item,
                BaseMatrixMut<Item, VectorContainer<Item>, $L, Dynamic, Dynamic>,
                $L,
                Dynamic,
                Dynamic,
            >
        {
            pub fn from_zeros(rows: IndexType, cols: IndexType) -> Self {
                Self::from_data(VectorContainer::<Item>::new(rows * cols), (rows, cols))
            }
        }
    };
}

from_zeros_dynamic_matrix!(CLayout);
from_zeros_dynamic_matrix!(FLayout);

// macro_rules! from_zeros_dynamic_vector {
//     ($RS:ident, $CS:ident) => {
//         impl<Item: Scalar>
//             MatrixMut<
//                 Item,
//                 BaseMatrixMut<Item, VectorContainer<Item>, VLayout, $RS, $CS>,
//                 VLayout,
//                 $RS,
//                 $CS,
//             >
//         {
//             pub fn from_zeros(nelems: IndexType) -> Self {
//                 let mut dim: (IndexType, IndexType) = (0, 0);
//                 if $RS::N == 0 {
//                     dim = (nelems, 1);
//                 }
//                 else {
//                     dim = (1, nelems);
//                 }
//                 Self::from_data(VectorContainer::<Item>::new(nelems), dim)
//             }
//         }
//     };
// }

// from_zeros_dynamic_vector!(Dynamic, Fixed1);
// from_zeros_dynamic_vector!(Fixed1, Dynamic);
