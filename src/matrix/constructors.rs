//! A collection of routines to construct matrix objects from scratch or existing data.

use crate::base_matrix::BaseMatrix;
use crate::data_container::{ArrayContainer, VectorContainer};
use crate::layouts::*;
use crate::matrix::{Matrix, RowVectorD, ColumnVectorD};
use crate::traits::*;
use crate::types::{IndexType, Scalar};

// Construct mutable zero matrices

macro_rules! from_zeros_fixed {
    ($RS:ident, $CS:ident, $L:ident) => {
        impl<Item: Scalar>
            Matrix<
                Item,
                BaseMatrix<Item, ArrayContainer<Item, { $RS::N * $CS::N }>, $L, $RS, $CS>,
                $L,
                $RS,
                $CS,
            >
        {
            pub fn zeros_from_dim() -> Self {
                Self::from_data(
                    ArrayContainer::<Item, { $RS::N * $CS::N }>::new(),
                    $L::from_dimension(($RS::N, $CS::N)),
                )
            }
        }
    };
}


macro_rules! from_zeros_fixed_vector {
    ($RS:ident, $CS:ident, $L:ident, $N:expr) => {
        impl<Item: Scalar>
            Matrix<
                Item,
                BaseMatrix<Item, ArrayContainer<Item, $N>, $L, $RS, $CS>,
                $L,
                $RS,
                $CS,
            >
        {
            pub fn zeros_from_length() -> Self {
                Self::from_data(
                    ArrayContainer::<Item, $N>::new(),
                    $L::from_length($N),
                )
            }
        }
    };
}

from_zeros_fixed!(Fixed2, Fixed2, RowMajor);
from_zeros_fixed!(Fixed1, Fixed2, RowMajor);

from_zeros_fixed!(Fixed3, Fixed3, RowMajor);
from_zeros_fixed!(Fixed1, Fixed3, RowMajor);

from_zeros_fixed!(Fixed2, Fixed3, RowMajor);
from_zeros_fixed!(Fixed3, Fixed2, RowMajor);

from_zeros_fixed!(Fixed2, Fixed2, ColumnMajor);
from_zeros_fixed!(Fixed1, Fixed2, ColumnMajor);

from_zeros_fixed!(Fixed3, Fixed3, ColumnMajor);
from_zeros_fixed!(Fixed1, Fixed3, ColumnMajor);

from_zeros_fixed!(Fixed2, Fixed3, ColumnMajor);
from_zeros_fixed!(Fixed3, Fixed2, ColumnMajor);

from_zeros_fixed_vector!(Fixed2, Fixed1, ColumnVector, 2);
from_zeros_fixed_vector!(Fixed3, Fixed1, ColumnVector, 3);
from_zeros_fixed_vector!(Fixed1, Fixed2, RowVector, 2);
from_zeros_fixed_vector!(Fixed1, Fixed3, RowVector, 3);



impl<Item: Scalar, L: BaseLayoutType>
    Matrix<Item, BaseMatrix<Item, VectorContainer<Item>, L, Dynamic, Dynamic>, L, Dynamic, Dynamic>
{
    pub fn zeros_from_dim(rows: IndexType, cols: IndexType) -> Self {
        let layout = L::from_dimension((rows, cols));
        Self::from_data(
            VectorContainer::<Item>::new(layout.number_of_elements()),
            L::from_dimension((rows, cols)),
        )
    }
}

impl<Item: Scalar>
    RowVectorD<Item>
{
    pub fn zeros_from_length(nelems: IndexType) -> Self {
        Self::from_data(
            VectorContainer::<Item>::new(nelems),
            RowVector::from_length(nelems),
        )
    }
}

impl<Item: Scalar>
    ColumnVectorD<Item>
{
    pub fn zeros_from_length(nelems: IndexType) -> Self {
        Self::from_data(
            VectorContainer::<Item>::new(nelems),
            ColumnVector::from_length(nelems),
        )
    }
}