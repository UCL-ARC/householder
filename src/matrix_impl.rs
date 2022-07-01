//! Implementation of matrix traits and methods

use crate::matrix::Matrix;
use crate::matrix_mut::MatrixMut;
use crate::traits::*;
use crate::types::{IndexType, Scalar};

macro_rules! methods {
    ($MatrixType:ident, $MatTrait:ident) => {
        impl<
                Item: Scalar,
                L: LayoutIdentifier,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
                MatImpl: $MatTrait<Item, L, RS, CS>,
            > LayoutIdentifier for $MatrixType<Item, MatImpl, L, RS, CS>
        {
        }
    };
}

methods!(Matrix, MatrixTrait);
methods!(MatrixMut, MatrixTraitMut);
