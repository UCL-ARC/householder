//! Layout Definitions

use crate::types::IndexType;

pub trait LayoutType {

    type IndexLayout: LayoutType; 

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

