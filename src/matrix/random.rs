//! Methods for the creation of random matrices.

use crate::data_container::DataContainerMut;
use crate::traits::*;
use crate::types::*;
use rand::prelude::*;
use rand_distr::{Distribution, StandardNormal};

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

// Random number implementations for the scalar types
trait RandScalar: Scalar {
    fn random_scalar<R: Rng, D: Distribution<Self::Real>>(rng: &mut R, dist: &D) -> Self;
}

impl RandScalar for f32 {
    fn random_scalar<R: Rng, D: Distribution<Self>>(rng: &mut R, dist: &D) -> Self {
        dist.sample(rng)
    }
}

impl RandScalar for f64 {
    fn random_scalar<R: Rng, D: Distribution<Self>>(rng: &mut R, dist: &D) -> Self {
        dist.sample(rng)
    }
}

impl RandScalar for c32 {
    fn random_scalar<R: Rng, D: Distribution<Self::Real>>(rng: &mut R, dist: &D) -> Self {
        c32::new(dist.sample(rng), dist.sample(rng))
    }
}

impl RandScalar for c64 {
    fn random_scalar<R: Rng, D: Distribution<Self::Real>>(rng: &mut R, dist: &D) -> Self {
        c64::new(dist.sample(rng), dist.sample(rng))
    }
}
