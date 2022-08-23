//! Layout Definitions
//! 
//! The traits in this module determine the memory layout of a matrix.
//! Consider a simple matrix of the form
//! \\[
//! A = \begin{bmatrix}1 & 2\\\\
//!                     3 & 4
//!      \end{bmatrix}
//! \\]
//! 
//! In **row-major** form this matrix is stored in memory in the order
//! $\begin{bmatrix}1 & 2 & 3 & 4\end{bmatrix}$ In **column-major** form the matrix is stored in memory
//! as $\begin{bmatrix}1 & 3 & 2 & 4\end{bmatrix}$. These are the two most commonly used
//! storage formats for matrices. A more general way of describing the memory ordering of matrix
//! elements is to introduce a stride tuple `(r, c)`. The meaning is that in memory we have
//! to walk `r` positions to go from one row to the next, and `c` positions to walk from one
//! column to the next. For the matrix $A$ the stride tuple is `(2, 1)` in **row-major**
//! and `(1, 2)` in **column-major** form.
//! 
//! Strides arise naturally in the description of submatrices.
//! Imagine that the matrix $A$ is a submatrix of the bigger matrix.
//! \\[
//! B = \begin{bmatrix}1 & 2 & x\\\\
//!                     3 & 4 & x\\\\
//!                     x & x & x
//!      \end{bmatrix},
//! \\]
//! where the $x$ are placeholders for some arbitrary numbers. A **row-major** layout of this matrix
//! is given as
//! \\[
//! \begin{bmatrix}1 & 2 & x & 3 & 4 & x & x & x & x\end{bmatrix}
//! \\]
//! If we wanted to describe the submatrix $A$ we could take the elements
//! $\begin{bmatrix}1 & 2 & x & 3 & 4\end{bmatrix}$. The stride tuple associated with this
//! layout is `(3, 1)`. The distance of elements within each row is `1` and the distance within
//! each column is `3`.
//! 
//! However, more complicated layouts are possible (e.g. a storage layout for upper triangular
//! matrices). The trait here ensures as much generality as possible in defining memory
//! layouts.
//! 
//! Each matrix is assigned with a logical indexing that is either **row-major** or **column-major**
//! and a physical layout. The logial indexing just determines whether a one-dimensional iterator
//! iterates through the matrix elements by row or by column. Consider again that $A$ is submatrix
//! of a larger $3\times 3$ matrix stored in **row-major** form. A logical **row-major** traversal
//! of $A$ will always return *\begin{bmatrix}1 & 2 & 3 & 4\end{bmatrix}$ independent of the
//! underlying physical layout.
//! 
//! The logical indexing is determined by the two methods [convert_1d_2d](crate::traits::LayoutType::convert_1d_2d)
//! and [convert_2d_1d](crate::traits::LayoutType::convert_2d_1d). These methods map between
//! logical `(row, col)` index tuples and one dimensional indices. The translation to the
//! underlying physical memory locations is handled by the routines [convert_1d_raw](crate::traits::LayoutType::convert_1d_raw)
//! and [convert_2d_raw](crate::traits::LayoutType::convert_2d_raw), which convert either
//! a two dimensional `(row, col)` index or a one-dimensional index to the raw physical location.
//! For base **row-major** and **column-major** storage types the physical and logical layout
//! are typical identical. But for more complex types (e.g. with arbitrary stride vectors) they
//! are typically different from each other.
//! 
//! The main trait in this module is the [LayoutType](crate::traits::LayoutType) trait.
//! If this is implemented for a matrix the [Layout](crate::traits::Layout) is auto-implemented.
//! This latter trait only provides a method to return the [LayoutType] implementation.
//! This crate also provides a number of other traits.
//! - [BaseLayoutType]: Derives from [LayoutType] 
//!   and marks simple base traits that are suitable 
//!   for logical indexing. Instantiations only depend on the matrix
//!   dimension and not e.g. non-standard strides.
//! - [VectorBaseLayoutType]: Derives from [BaseLayoutType]
//!   and marks base layouts for vectors. Only requires the
//!   length of the vector for instantiation.
//! - [MatrixBaseLayoutType]: Derives from [BaseLayoutType]
//!   and marks base layouts for matrices.
//! - [StridedLayoutType]: Derives from [LayoutType] and
//!   marks layouts with non-trivial strides.

use crate::types::IndexType;

pub trait BaseLayoutType: LayoutType {
    fn from_dimension(dim: (IndexType, IndexType)) -> Self;
}

pub trait VectorBaseLayoutType: BaseLayoutType {
    fn from_length(length: IndexType) -> Self;
}

pub trait MatrixBaseLayoutType: BaseLayoutType {}

pub trait StridedLayoutType: LayoutType {}

pub trait LayoutType {

    type IndexLayout: BaseLayoutType;

    fn stride(&self) -> (IndexType, IndexType);
    fn dim(&self) -> (IndexType, IndexType);
    fn number_of_elements(&self) -> IndexType;

    fn convert_1d_2d(&self, index: IndexType) -> (IndexType, IndexType);

    fn convert_2d_1d(&self, row: IndexType, col: IndexType) -> IndexType;

    fn convert_1d_raw(&self, index: IndexType) -> IndexType;

    fn convert_2d_raw(&self, row: IndexType, col: IndexType) -> IndexType;

    fn index_layout(&self) -> Self::IndexLayout;
}


pub trait Layout {
    type Impl: LayoutType;

    fn layout(&self) -> &Self::Impl;
}
