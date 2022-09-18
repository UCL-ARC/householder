//! This module combines different concrete memory layout types.
//! 
//! Memory layouts implement the [LayoutType](crate::traits::LayoutType) trait
//! and describe iteration order of matrix elements and their map to memory
//! locations.

pub mod row_major;
pub mod column_major;
pub mod upper_triangular;
pub mod column_vector;
pub mod row_vector;
pub mod arbitrary_stride_row_major;
pub mod arbitrary_stride_column_major;
pub mod arbitrary_stride_row_vector;
pub mod arbitrary_stride_column_vector;



pub use row_major::*;
pub use column_major::*;
pub use upper_triangular::*;
pub use column_vector::*;
pub use row_vector::*;
pub use arbitrary_stride_row_major::*;
pub use arbitrary_stride_column_major::*;
pub use arbitrary_stride_row_vector::*;
pub use arbitrary_stride_column_vector::*;

