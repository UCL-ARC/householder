//! Definition of typical memory layouts
use crate::traits::*;
use crate::types::IndexType;

pub struct RowMajor {
    dim: (IndexType, IndexType),
}

pub struct ColumnMajor {
    dim: (IndexType, IndexType),
}

pub struct ColumnVector {
    dim: IndexType,
}

pub struct RowVector {
    dim: IndexType,
}

pub struct ArbitraryStrideColumnVector {
    dim: IndexType,
    stride: IndexType,
}

pub struct ArbitraryStrideRowVector {
    dim: IndexType,
    stride: IndexType,
}

pub struct ArbitraryStrideRowMajor {
    dim: (IndexType, IndexType),
    stride: (IndexType, IndexType),
}

pub struct ArbitraryStrideColumnMajor {
    dim: (IndexType, IndexType),
    stride: (IndexType, IndexType),
}

impl RowMajor {
    pub fn new(dim: (IndexType, IndexType)) -> Self {
        Self { dim }
    }
}

impl ColumnMajor {
    pub fn new(dim: (IndexType, IndexType)) -> Self {
        Self { dim }
    }
}

impl ArbitraryStrideRowMajor {
    pub fn new(dim: (IndexType, IndexType), stride: (IndexType, IndexType)) -> Self {
        Self { dim, stride }
    }
}

impl ArbitraryStrideColumnMajor {
    pub fn new(dim: (IndexType, IndexType), stride: (IndexType, IndexType)) -> Self {
        Self { dim, stride }
    }
}

impl ColumnVector {
    pub fn new(dim: IndexType) -> Self {
        Self { dim }
    }
}

impl RowVector {
    pub fn new(dim: IndexType) -> Self {
        Self { dim }
    }
}

impl ArbitraryStrideColumnVector {
    pub fn new(dim: IndexType, stride: IndexType) -> Self {
        Self { dim, stride }
    }
}

impl ArbitraryStrideRowVector {
    pub fn new(dim: IndexType, stride: IndexType) -> Self {
        Self { dim, stride }
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

impl LayoutType for ColumnMajor {
    type IndexLayout = ColumnMajor;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (index % self.dim.0, index / self.dim.0)
    }

    #[inline]
    fn convert_2d_1d(&self, row: IndexType, col: IndexType) -> IndexType {
        col * self.dim.0 + row
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
        (1, self.dim.0)
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

impl LayoutType for ArbitraryStrideRowMajor {
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
        self.stride.0 * row + self.stride.1 * col
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        let (row, col) = self.convert_1d_2d(index);
        self.convert_2d_raw(row, col)
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

impl LayoutType for ArbitraryStrideColumnMajor {
    type IndexLayout = ColumnMajor;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (index % self.dim.0, index / self.dim.0)
    }

    #[inline]
    fn convert_2d_1d(&self, row: IndexType, col: IndexType) -> IndexType {
        col * self.dim.0 + row
    }

    #[inline]
    fn convert_2d_raw(&self, row: IndexType, col: IndexType) -> IndexType {
        self.stride.0 * row + self.stride.1 * col
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        let (row, col) = self.convert_1d_2d(index);
        self.convert_2d_raw(row, col)
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        self.dim
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (1, self.dim.0)
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

impl LayoutType for ColumnVector {
    type IndexLayout = ColumnVector;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (index, 1)
    }

    #[inline]
    fn convert_2d_1d(&self, row: IndexType, _col: IndexType) -> IndexType {
        row
    }

    #[inline]
    fn convert_2d_raw(&self, row: IndexType, _col: IndexType) -> IndexType {
        row
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        (self.dim, 1)
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (1, 1)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim().0)
    }

}

impl LayoutType for RowVector {
    type IndexLayout = RowVector;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (1, index)
    }

    #[inline]
    fn convert_2d_1d(&self, _row: IndexType, col: IndexType) -> IndexType {
        col
    }

    #[inline]
    fn convert_2d_raw(&self, _row: IndexType, col: IndexType) -> IndexType {
        col
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        (1, self.dim)
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (1, 1)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim().1)
    }

}

//--------------

impl LayoutType for ArbitraryStrideColumnVector {
    type IndexLayout = ColumnVector;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (index, 1)
    }

    #[inline]
    fn convert_2d_1d(&self, row: IndexType, _col: IndexType) -> IndexType {
        row
    }

    #[inline]
    fn convert_2d_raw(&self, row: IndexType, _col: IndexType) -> IndexType {
        row * self.stride
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index * self.stride
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        (self.dim, 1)
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (self.stride, 1)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim().0)
    }

}

impl LayoutType for ArbitraryStrideRowVector {
    type IndexLayout = RowVector;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        (1, index)
    }

    #[inline]
    fn convert_2d_1d(&self, _row: IndexType, col: IndexType) -> IndexType {
        col
    }

    #[inline]
    fn convert_2d_raw(&self, _row: IndexType, col: IndexType) -> IndexType {
        col * self.stride
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index * self.stride
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        (1, self.dim)
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        (1, self.stride)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        self.dim
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim().1)
    }

}

impl BaseLayoutType for RowMajor {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self {
        Self { dim }
    }
}

impl BaseLayoutType for ColumnMajor {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self {
        Self { dim }
    }
}

impl BaseLayoutType for RowVector {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self {
        assert_eq!(
            dim.0, 1,
            "Number of rows is {} but must be one for RowVector.",
            dim.0
        );
        Self { dim: dim.1 }
    }
}
impl BaseLayoutType for ColumnVector {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self {
        assert_eq!(
            dim.0, 1,
            "Number of columns is {} but must be one for ColumnVector.",
            dim.1
        );
        Self { dim: dim.0 }
    }
}

impl MatrixBaseLayoutType for RowMajor {}

impl MatrixBaseLayoutType for ColumnMajor {}

impl VectorBaseLayoutType for RowVector {
    fn from_length(length: IndexType) -> Self {
        Self { dim: length }
    }
}

impl VectorBaseLayoutType for ColumnVector {
    fn from_length(length: IndexType) -> Self {
        Self { dim: length }
    }
}
