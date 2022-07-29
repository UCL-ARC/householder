//! Upper triangular layout

use crate::traits::*;
use crate::types::IndexType;

pub struct UpperTriangular {
    dim: IndexType,
}

impl UpperTriangular {
    pub fn new(dim: IndexType) -> Self {
        Self { dim }
    }
}

impl LayoutType for UpperTriangular {
    type IndexLayout = Self;

    #[inline]
    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType) {
        let p = -0.5 + f64::sqrt(0.25 + 2.0 * (index as f64));
        let p = f64::floor(p) as IndexType;
        (
            self.dim - 1 - p,
            self.dim - 1 - p + index - (p * (p + 1)) / 2,
        )
    }

    #[inline]
    fn convert_1d_raw(&self, index: IndexType) -> IndexType {
        index
    }

    #[inline]
    fn convert_2d_1d(&self, row: IndexType, col: IndexType) -> IndexType {
        assert!(col >= row, "For upper triangular require 'col' >= 'row': row={}, col={}", row, col);
        ((self.dim - row) * (self.dim - row - 1)) / 2 + col - row
    }

    #[inline]
    fn convert_2d_raw(&self, row: IndexType, col: IndexType) -> IndexType {
        self.convert_1d_raw(self.convert_2d_1d(row, col))
    }

    #[inline]
    fn dim(&self) -> (IndexType, IndexType) {
        (self.dim, self.dim)
    }

    #[inline]
    fn index_layout(&self) -> Self::IndexLayout {
        Self::IndexLayout::new(self.dim)
    }

    #[inline]
    fn number_of_elements(&self) -> IndexType {
        (self.dim * (self.dim + 1)) / 2
    }

    #[inline]
    fn stride(&self) -> (IndexType, IndexType) {
        std::unimplemented!("method 'stride' not implemented for UpperTriangular layout.")
    }
}

impl BaseLayoutType for UpperTriangular {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self {
        assert_eq!(
            dim.0, dim.1,
            "Only square triangular matrices are supported. dim = {:#?}",
            dim
        );
        Self { dim: dim.0 }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_upper_triangular_indexing() {
        let n = 5;

        let layout = UpperTriangular::new(n);

        let index = layout.convert_2d_1d(n - 1, n - 1);
        assert_eq!(index, 0);

        let index = layout.convert_2d_1d(n - 2, n - 1);
        assert_eq!(index, 2);

        let (row, col) = layout.convert_1d_2d(0);
        assert_eq!((row, col), (n - 1, n - 1));

        let (row, col) = layout.convert_1d_2d(2);
        assert_eq!((row, col), (n - 2, n - 1));

        let (row, col) = layout.convert_1d_2d((n * (n + 1)) / 2 - 1);
        assert_eq!((row, col), (0, n - 1));
    }
}
