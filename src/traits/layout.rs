//! Layout Definitions

use crate::types::IndexType;

pub trait BaseLayoutType: LayoutType {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self;
}

pub trait VectorBaseLayoutType: BaseLayoutType {
    fn from_length(length: IndexType) -> Self;
}

pub trait MatrixBaseLayoutType: BaseLayoutType {}

pub trait LayoutType {
    type IndexLayout: BaseLayoutType;

    fn stride(&self) -> (IndexType, IndexType);
    fn dim(&self) -> (IndexType, IndexType);
    fn number_of_elements(&self) -> IndexType;

    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType);

    fn convert_2d_1d(&self, row: IndexType, col: IndexType) -> IndexType;

    fn convert_1d_raw(&self, index: IndexType) -> IndexType;

    fn convert_2d_raw(&self, row: IndexType, col: IndexType) -> IndexType;
}

pub trait Layout {
    type Impl: LayoutType;

    fn layout(&self) -> &Self::Impl;
}
