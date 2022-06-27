//! Layout Definitions

use crate::types::IndexType;
use crate::traits::Dimensions;

pub struct CLayout;
pub struct FLayout;

pub struct CustomLayout;

// Separate Layout for vectors
pub struct VLayout;

pub trait Stride: Dimensions {
    fn row_stride(&self) -> IndexType;
    fn column_stride(&self) -> IndexType;
}

pub trait LayoutIdentifier {}

impl LayoutIdentifier for CLayout {}
impl LayoutIdentifier for FLayout {}
impl LayoutIdentifier for CustomLayout {}

pub trait LayoutType<L: LayoutIdentifier> {}