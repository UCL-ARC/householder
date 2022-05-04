//! Basic traits for matrices
use cauchy::Scalar;

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryLayout {
    C,
    F,
    V,
}

pub enum MatrixSizeType {
    MATRIX2,
    MATRIX3,
    MATRIXD,
}

/// Bounds checked random access for matrices.
pub trait SafeRandomAccess {
    type Output: Scalar;

    /// Get the element at position (row, col) of the matrix.
    fn get(&self, row: usize, col: usize) -> Self::Output;

    /// Get element from matrix linearized as 1d array (result depends on memory layout).
    fn get1d(&self, elem: usize) -> Self::Output;
}

/// Bounds checked mutable random access for matrices.
pub trait SafeMutableRandomAccess {
    type Output: Scalar;

    /// Get mutable reference to element at position (row, col) of the matrix.
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output;
    /// Get mutable reference from matrix linearized as 1d array (result depends on memory layout).
    fn get1d_mut(&mut self, elem: usize) -> &mut Self::Output;
}

/// Random access without bounds check for matrices.
pub trait UnsafeRandomAccess {
    type Output: Scalar;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output;
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output;
}

/// Get mutable access to element without bounds check.
pub trait UnsafeMutableRandomAccess {
    type Output: Scalar;

    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output;
    unsafe fn get1d_unchecked_mut(&mut self, index: usize) -> &mut Self::Output;
}

/// General trait specifying random access
pub trait RandomAccess<Item: Scalar>:
    SafeRandomAccess<Output = Item> + UnsafeRandomAccess<Output = Item>
{
}

impl<Item: Scalar, T: SafeRandomAccess<Output = Item> + UnsafeRandomAccess<Output = Item>>
    RandomAccess<Item> for T
{
}

/// General trait specifying random access that can also be mutable
pub trait RandomAccessMut<Item: Scalar>:
    SafeRandomAccess<Output = Item>
    + UnsafeRandomAccess<Output = Item>
    + SafeMutableRandomAccess<Output = Item>
    + UnsafeMutableRandomAccess<Output = Item>
{
}

impl<
        Item: Scalar,
        T: SafeRandomAccess<Output = Item>
            + UnsafeRandomAccess<Output = Item>
            + SafeMutableRandomAccess<Output = Item>
            + UnsafeMutableRandomAccess<Output = Item>,
    > RandomAccessMut<Item> for T
{
}

// Data types to specify types of fixed size or dynamic matrices

/// Fixed size 2x2 matrix
pub struct Matrix2;

/// Fixed size 3x3 matrix
pub struct Matrix3;

/// Dynamic sized matrix
pub struct MatrixD;

// Length 2 row vector
pub struct RowVector2;

// Length 3 row vector
pub struct RowVector3;

// Dynamix length row vector
pub struct RowVectorD;

// Length 2 col vector
pub struct ColVector2;

// Length 3 col vector
pub struct ColVector3;

// Dynamix length col vector
pub struct ColVectorD;

pub trait SizeIdentifier {}

impl SizeIdentifier for Matrix2 {}
impl SizeIdentifier for Matrix3 {}
impl SizeIdentifier for MatrixD {}

pub trait SizeType {
    type S: SizeIdentifier;
}

pub trait Size<S: SizeIdentifier>: SizeType<S = S> {
    // Return the size type of the object
    fn size_type(&self) -> MatrixSizeType;
}

impl<T: SizeType<S = Matrix2>> Size<Matrix2> for T {
    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::MATRIX2
    }
}

impl<T: SizeType<S = Matrix3>> Size<Matrix3> for T {
    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::MATRIX3
    }
}

impl<T: SizeType<S = MatrixD>> Size<MatrixD> for T {
    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::MATRIXD
    }
}

// The following specifies traits to mark matrices as having either a C or Fortran
// Layout. We do this on the type level and not via runtime information so the
// compiler can distinguish between these two.

// These are empty types specifying whether we have C or Fortran Layout
pub struct CLayout;
pub struct FLayout;

// Separate Layout for vectors
pub struct VLayout;

// Marker trait to specify layout identifiers
pub trait LayoutIdentifier {
    /// Tranform (row, col) index to one dimensional index.
    fn transform_index(row: usize, col: usize, dim: (usize, usize)) -> usize;
}

impl LayoutIdentifier for CLayout {
    #[inline]
    fn transform_index(row: usize, col: usize, dim: (usize, usize)) -> usize {
        row * dim.1 + col
    }
}
impl LayoutIdentifier for FLayout {
    #[inline]
    fn transform_index(row: usize, col: usize, dim: (usize, usize)) -> usize {
        col * dim.0 + row
    }
}
impl LayoutIdentifier for VLayout {

    // Choosing C Layout here. Fortran and C Layout are identical for vectors
    // and both are possible.
    #[inline]
    fn transform_index(row: usize, col: usize, dim: (usize, usize)) -> usize {
        row * dim.1 + col
    }
}

/// A generic trait to obtain memory layout information

pub trait LayoutType<L> {}

pub trait Layout<L: LayoutIdentifier>: LayoutType<L> {
    fn layout_type(&self) -> MemoryLayout;
}

impl<T: LayoutType<CLayout>> Layout<CLayout> for T {
    fn layout_type(&self) -> MemoryLayout {
        MemoryLayout::C
    }
}

impl<T: LayoutType<FLayout>> Layout<FLayout> for T {
    fn layout_type(&self) -> MemoryLayout {
        MemoryLayout::F
    }
}

impl<T: LayoutType<VLayout>> LayoutType<CLayout> for T {}
impl<T: LayoutType<VLayout>> LayoutType<FLayout> for T {}

/// Any matrix type that has an associated dimension.
pub trait Dimensions {
    /// Return a tuple (row, col) specifying the dimension of the matrix.
    fn dim(&self) -> (usize, usize);
}

/// Implement this trait to provide an iterator
pub trait Iterable<'a, Item: Scalar, Iter: Iterator<Item = Item>> {
    fn iter(&'a self) -> Iter;
}

// Implement this trait to provide a mutable
pub trait IterableMut<'a, Item: Scalar, Iter: Iterator<Item = &'a mut Item>> {
    fn iter_mut(&'a mut self) -> Iter;
}

/// Combined trait that summarizes basic matrix properties
pub trait MatrixTrait<'a, Item: Scalar, Layout: LayoutIdentifier, Size: SizeIdentifier>:
    RandomAccess<Item> + Dimensions + LayoutType<Layout> + SizeType<S = Size>
{
}

/// Combined trait for mutable matrices
pub trait MatrixMutTrait<'a, Item: Scalar, Layout: LayoutIdentifier, Size: SizeIdentifier>:
    RandomAccess<Item> + Dimensions + LayoutType<Layout> + SizeType<S = Size>
{
}

/// Access to a raw pointer containing the data
pub trait Pointer {
    type Item: Scalar;

    fn as_ptr(&self) -> *const Self::Item;
}

/// Mutable access to a raw pointer cointaing the data
pub trait PointerMut {
    type Item: Scalar;

    fn as_mut_ptr(&mut self) -> *mut Self::Item;
}

// Implement `MatrixTrait` for any eligible object.
impl<'a, Item, Layout, Size, Mat> MatrixTrait<'a, Item, Layout, Size> for Mat
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Mat: RandomAccess<Item> + Dimensions + LayoutType<Layout> + SizeType<S = Size>,
{
}

// Implement `MatrixTraitMut` for any eligible object.
impl<'a, Item, Layout, Size, Mat> MatrixMutTrait<'a, Item, Layout, Size> for Mat
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Mat: RandomAccessMut<Item> + Dimensions + LayoutType<Layout> + SizeType<S = Size>,
{
}

/// Length of a vector
pub trait VectorLength {
    fn len(&self) -> usize;
}
