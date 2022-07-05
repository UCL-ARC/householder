//! Specialisation for vectors
//!

use crate::base_matrix_mut::BaseMatrixMut;
use crate::data_container::{ArrayContainer, VectorContainer};
use crate::matrix::MatrixMut;
use crate::traits::*;
use crate::types::{IndexType, Scalar};

macro_rules! vec_from_data {
    ($MatrixType:ident, $BaseType:ident, $RS:ident, $CS:ident) => {
        impl<Item: Scalar>
            MatrixMut<
                Item,
                BaseMatrixMut<Item, VectorContainer<Item>, VLayout, $RS, $CS>,
                VLayout,
                $RS,
                $CS,
            >
        {
            pub fn from_data(data: VectorContainer<Item>) -> Self {

                Self::new(BaseMatrixMut::<
                    Item,
                    VectorContainer<Item>,
                    VLayout,
                    $RS,
                    $CS,
                >::new(data))
            }
        }
    };
}

vec_from_data!(Matrix, BaseMatrix, Dynamic, Fixed1);

// impl<Item: Scalar>
//     MatrixMut<
//         Item,
//         BaseMatrixMut<Item, ArrayContainer<Item, 3>, VLayout, Dynamic, Fixed1>,
//         VLayout,
//         Dymamic,
//         Fixed1,
//     >
// {
//     pub fn from_zeros() -> Self {
//         Self::from_data(ArrayContainer<Item, 3>)
//     }
// }
