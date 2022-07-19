//! A Rust native linear algebra library

pub mod data_container;
pub mod types;
pub mod traits;
pub mod base_matrix;
pub mod matrix;
pub mod matrix_ref;
pub mod scalar_mult;

//pub mod traits;
//pub mod slice_matrix;
//pub mod matrix;
//pub mod base_types;
//pub mod matrix_operators;
//pub mod macros;
//pub mod iterators;
//pub mod scalar_mult;
//pub mod addition;
//pub mod matrix_multiply;

pub use cauchy::Scalar;
