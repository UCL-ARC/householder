//! Methods for the creation of random matrices.

use crate::data_container::DataContainerMut;
use crate::traits::*;
use crate::types::*;
use rand::prelude::*;
use rand_distr::StandardNormal;
use crate::tools::*;

use super::GenericBaseMatrixMut;

macro_rules! rand_impl {
    ($Scalar:ty) => {
        impl<
                L: LayoutType,
                RS: SizeIdentifier,
                CS: SizeIdentifier,
                Data: DataContainerMut<Item = $Scalar>,
            > GenericBaseMatrixMut<$Scalar, L, Data, RS, CS>
        {
            pub fn fill_from_rand_standard_normal<R: Rng>(&mut self, rng: &mut R) {
                let dist = StandardNormal;
                self.for_each(|val| *val = <$Scalar>::random_scalar(rng, &dist));
            }
        }
    };
}

rand_impl!(f32);
rand_impl!(f64);
rand_impl!(c32);
rand_impl!(c64);

