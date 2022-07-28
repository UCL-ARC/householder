//! Various useful tools

use crate::types::*;
use rand::prelude::*;
use rand_distr::Distribution;

// Random number implementations for the scalar types
pub trait RandScalar: Scalar {
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
