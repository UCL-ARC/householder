//! Layout Definitions

use crate::traits::Dimensions;
use crate::types::IndexType;

pub struct CLayout;
pub struct FLayout;

pub struct StrideCLayout;
pub struct StrideFLayout;

// Separate Layout for vectors
pub struct VLayout;
pub struct StrideVLayout;

pub trait Stride: Dimensions {
    fn row_stride(&self) -> IndexType;
    fn column_stride(&self) -> IndexType;
}

pub trait LayoutIdentifier {
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> IndexType;
    fn get_raw_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> IndexType;
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> IndexType;
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType);
}

impl LayoutIdentifier for CLayout {
    #[inline]
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        row * dim.1 + col
    }

    #[inline]
    fn get_raw_index_from_1d_index(
            index: IndexType,
            _dim: (IndexType, IndexType),
            _stride: (IndexType, IndexType),
        ) -> IndexType {
        index
    }

    #[inline]
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        row * dim.1 + col
    }

    #[inline]
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType) {
        (index / dim.1, index % dim.1)
    }
}

impl LayoutIdentifier for FLayout {
    #[inline]
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        col * dim.0 + row
    }

    #[inline]
    fn get_raw_index_from_1d_index(
            index: IndexType,
            _dim: (IndexType, IndexType),
            _stride: (IndexType, IndexType),
        ) -> IndexType {
        index
    }

    #[inline]
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        col * dim.0 + row
    }

    #[inline]
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType) {
        (index % dim.0, index / dim.0)
    }
}

impl LayoutIdentifier for StrideCLayout {
    #[inline]
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        _dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> IndexType {
        row * stride.0 + col * stride.1 
    }

    #[inline]
    fn get_raw_index_from_1d_index(
            index: IndexType,
            dim: (IndexType, IndexType),
            stride: (IndexType, IndexType),
        ) -> IndexType {
        let (row, col) = Self::get_2d_index_from_1d_index(index, dim, stride);
        Self::get_raw_index_from_2d_index(
            row, col, dim, stride
        )
    }

    #[inline]
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        row * dim.1 + col
    }

    #[inline]
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType) {
        (index / dim.1, index % dim.1)
    }
}

impl LayoutIdentifier for StrideFLayout {
    #[inline]
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        _dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> IndexType {
        row * stride.0 + col * stride.1 
    }

    #[inline]
    fn get_raw_index_from_1d_index(
            index: IndexType,
            dim: (IndexType, IndexType),
            stride: (IndexType, IndexType),
        ) -> IndexType {
        let (row, col) = Self::get_2d_index_from_1d_index(index, dim, stride);
        Self::get_raw_index_from_2d_index(
            row, col, dim, stride
        )
    }

    #[inline]
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        col * dim.0 + row
    }

    #[inline]
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType) {
        (index % dim.0, index / dim.0)
    }
}


impl LayoutIdentifier for VLayout {
    #[inline]
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        row * dim.1 + col
    }

    #[inline]
    fn get_raw_index_from_1d_index(
            index: IndexType,
            _dim: (IndexType, IndexType),
            _stride: (IndexType, IndexType),
        ) -> IndexType {
        index
    }

    #[inline]
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        row * dim.1 + col
    }

    #[inline]
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType) {
        (index / dim.1, index % dim.1)
    }
}

impl LayoutIdentifier for StrideVLayout {
    #[inline]
    fn get_raw_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        _dim: (IndexType, IndexType),
        stride: (IndexType, IndexType),
    ) -> IndexType {
        row * stride.0 + col * stride.1 
    }

    #[inline]
    fn get_raw_index_from_1d_index(
            index: IndexType,
            dim: (IndexType, IndexType),
            stride: (IndexType, IndexType),
        ) -> IndexType {
        let (row, col) = Self::get_2d_index_from_1d_index(index, dim, stride);
        Self::get_raw_index_from_2d_index(
            row, col, dim, stride
        )
    }

    #[inline]
    fn get_1d_index_from_2d_index(
        row: IndexType,
        col: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> IndexType {
        row * dim.1 + col
    }

    #[inline]
    fn get_2d_index_from_1d_index(
        index: IndexType,
        dim: (IndexType, IndexType),
        _stride: (IndexType, IndexType),
    ) -> (IndexType, IndexType) {
        (index / dim.1, index % dim.1)
    }
}



pub trait LayoutType<L: LayoutIdentifier> {}
