//! The main matrix class.
//!

pub mod common_impl;
pub mod constructors;
//pub mod vector_impl;
pub mod base_methods;
pub mod random;
pub mod matrix_slices;

use crate::base_matrix::BaseMatrix;
use crate::data_container::{ArrayContainer, DataContainer, VectorContainer};
use crate::matrix_ref::MatrixRef;
use crate::traits::*;
use crate::types::Scalar;
use std::marker::PhantomData;

pub type RefMat<'a, Item, MatImpl, L, RS, CS> =
    Matrix<Item, MatrixRef<'a, Item, MatImpl, L, RS, CS>, L, RS, CS>;

pub type GenericBaseMatrix<Item, L, Data, RS, CS> =
    Matrix<Item, BaseMatrix<Item, Data, L, RS, CS>, L, RS, CS>;

pub type GenericBaseMatrixMut<Item, L, Data, RS, CS> =
    Matrix<Item, BaseMatrix<Item, Data, L, RS, CS>, L, RS, CS>;

pub type MatrixD<Item, L> =
    Matrix<Item, BaseMatrix<Item, VectorContainer<Item>, L, Dynamic, Dynamic>, L, Dynamic, Dynamic>;

pub type Matrix22<Item, L> =
    Matrix<Item, BaseMatrix<Item, ArrayContainer<Item, 4>, L, Fixed2, Fixed2>, L, Fixed2, Fixed2>;

pub type Matrix33<Item, L> =
    Matrix<Item, BaseMatrix<Item, ArrayContainer<Item, 9>, L, Fixed3, Fixed3>, L, Fixed3, Fixed3>;

pub type Matrix32<Item, L> =
    Matrix<Item, BaseMatrix<Item, ArrayContainer<Item, 6>, L, Fixed3, Fixed2>, L, Fixed3, Fixed2>;

pub type Matrix23<Item, L> =
    Matrix<Item, BaseMatrix<Item, ArrayContainer<Item, 6>, L, Fixed2, Fixed3>, L, Fixed2, Fixed3>;

pub struct Matrix<Item, MatImpl, L, RS, CS>(
    MatImpl,
    PhantomData<Item>,
    PhantomData<L>,
    PhantomData<RS>,
    PhantomData<CS>,
)
where
    Item: Scalar,
    L: LayoutType,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    MatImpl: MatrixTrait<Item, L, RS, CS>;

impl<
        Item: Scalar,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        MatImpl: MatrixTrait<Item, L, RS, CS>,
    > Matrix<Item, MatImpl, L, RS, CS>
{
    pub fn new(mat: MatImpl) -> Self {
        Self(mat, PhantomData, PhantomData, PhantomData, PhantomData)
    }

    pub fn from_ref<'a>(
        mat: &'a Matrix<Item, MatImpl, L, RS, CS>,
    ) -> RefMat<'a, Item, MatImpl, L, RS, CS> {
        RefMat::new(MatrixRef::new(mat))
    }
}

impl<
        Item: Scalar,
        L: LayoutType,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
        Data: DataContainer<Item = Item>,
    > Matrix<Item, BaseMatrix<Item, Data, L, RS, CS>, L, RS, CS>
{
    pub fn from_data(data: Data, layout: L) -> Self {
        Self::new(BaseMatrix::<Item, Data, L, RS, CS>::new(data, layout))
    }
}

// #[cfg(test)]
// mod test {

//     use super::*;
//     use crate::col_vec;
//     use crate::mat;

//     #[test]
//     fn scalar_mult_matrix() {
//         let mut mat1 = mat![f64, (2, 3), FLayout];
//         let mut mat2 = mat![f64, (2, 3), FLayout];

//         *mat1.get_mut(1, 2) = 2.0;
//         *mat2.get_mut(1, 2) = 3.0;

//         let res = 5.0 * &mat1 + mat2;

//         assert_eq!(res.get(1, 2), 13.0);
//     }

//     #[test]
//     fn scalar_mult_vector() {
//         let mut vec1 = col_vec![f64, 5];
//         let mut vec2 = col_vec![f64, 5];

//         *vec1.get1d_mut(2) = 2.0;
//         *vec2.get1d_mut(2) = 3.0;

//         let res = 5.0 * vec1 + &vec2;

//         assert_eq!(res.get1d(2), 13.0);
//     }
// }
