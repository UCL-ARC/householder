//! Row major layout

use crate::traits::*;
use crate::types::IndexType;

pub struct RowMajor {
    dim: (IndexType, IndexType),
}

impl RowMajor {
    pub fn new(dim: (IndexType, IndexType)) -> Self {
        Self { dim }
    }
}

impl LayoutType for RowMajor {
    type IndexLayout = RowMajor;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (index / self.dim.1, index % self.dim.1)
    }

    #[inline]
    fn convert_2d_1d(&self, row: IndexType, col: IndexType) -> IndexType {
        row * self.dim.1 + col
    }

    #[inline]
    fn convert_2d_raw(&self, row: IndexType, col: IndexType) -> IndexType {
        self.convert_2d_1d(row, col)
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        self.dim
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (self.dim.1, 1)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim.0 * self.dim.1
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim())
    }
}

impl BaseLayoutType for RowMajor {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self {
        Self { dim }
    }
}

impl MatrixBaseLayoutType for RowMajor {}
impl StridedLayoutType for RowMajor {}
