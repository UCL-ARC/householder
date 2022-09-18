//! Scalar types used by the library

/// The [Scalar] trait describes scalar floating types. It is implemented
/// for [f32], [f64], [c32], [c64].
pub use cauchy::Scalar;

/// Single precision complex type.
pub use cauchy::c32;

/// Double precision complex type.
pub use cauchy::c64;

/// The index type used throughout this crate. By default
/// it is set to `usize`.
pub type IndexType = usize;


