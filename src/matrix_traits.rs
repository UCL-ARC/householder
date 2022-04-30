//! Basic traits for matrices
use cauchy::Scalar;

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryLayout {
    C,
    F,
    CUSTOM,
}

pub enum MatrixSizeType {
    MATRIX2,
    MATRIX3,
    MATRIXD,
    VECTOR2,
    VECTOR3,
    VECTORD,
}

/// Bounds checked random access for matrices.
pub trait SafeRandomAccess {
    type Output: Scalar;

    /// Get the element at position (row, col) of the matrix.
    fn get(&self, row: usize, col: usize) -> Self::Output;
}

/// Bounds checked mutable random access for matrices.
pub trait SafeMutableRandomAccess {
    type Output: Scalar;

    /// Get mutable reference to element at position (row, col) of the matrix.
    fn get_mut(&mut self, row: usize, col: usize) -> &mut Self::Output;
}

/// Random access without bounds check for matrices.
pub trait UnsafeRandomAccess {
    type Output: Scalar;

    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output;
}

/// Get mutable access to element without bounds check.
pub trait UnsafeMutableRandomAccess {
    type Output: Scalar;

    unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut Self::Output;
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

// Length 2 vector
pub struct Vector2;

// Length 3 vector
pub struct Vector3;

// Dynamix length vector
pub struct VectorD;

pub trait SizeIdentifier {}

impl SizeIdentifier for Matrix2 {}
impl SizeIdentifier for Matrix3 {}
impl SizeIdentifier for MatrixD {}

impl SizeIdentifier for Vector2 {}
impl SizeIdentifier for Vector3 {}
impl SizeIdentifier for VectorD {}

pub trait SizeType {
    type S: SizeIdentifier;

}

pub trait Size<S: SizeIdentifier>: SizeType<S=S> {

    // Return the size type of the object
    fn size_type(&self) -> MatrixSizeType;
}

impl<T: SizeType<S=Matrix2>> Size<Matrix2> for T {

    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::MATRIX2
    }
}

impl<T: SizeType<S=Matrix3>> Size<Matrix3> for T {

    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::MATRIX3
    }
}

impl<T: SizeType<S=MatrixD>> Size<MatrixD> for T {

    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::MATRIXD
    }
}

impl<T: SizeType<S=Vector2>> Size<Vector2> for T {

    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::VECTOR2
    }
}

impl<T: SizeType<S=Vector3>> Size<Vector3> for T {

    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::VECTOR3
    }
}

impl<T: SizeType<S=VectorD>> Size<VectorD> for T {

    fn size_type(&self) -> MatrixSizeType {
        MatrixSizeType::VECTORD
    }
}



// The following specifies traits to mark matrices as having either a C or Fortran
// Layout. We do this on the type level and not via runtime information so the
// compiler can distinguish between these two.

// These are empty types specifying whether we have C or Fortran Layout
pub struct CLayout;
pub struct FortranLayout;

pub struct CustomLayout;

// Marker trait to specify layout identifiers
pub trait LayoutIdentifier {}

impl LayoutIdentifier for CLayout {}
impl LayoutIdentifier for FortranLayout {}
impl LayoutIdentifier for CustomLayout {}

/// A generic trait to obtain memory layout information

pub trait LayoutType {
    type L: LayoutIdentifier;

}

pub trait Layout<L: LayoutIdentifier>: LayoutType<L=L> {

    fn layout_type(&self) -> MemoryLayout;

}

impl<T: LayoutType<L=CLayout>> Layout<CLayout> for T {
    fn layout_type(&self) -> MemoryLayout {
        MemoryLayout::C
    }
}

impl<T: LayoutType<L=FortranLayout>> Layout<FortranLayout> for T {
    fn layout_type(&self) -> MemoryLayout {
        MemoryLayout::F
    }
}



// pub trait Layout<T: LayoutType>: LayoutHandler<T> {
//     fn layout(&self) -> MemoryLayout {
//         self._layout_impl()
//     }
// }

// impl<T: LayoutType + LayoutHandler<T>> Layout<T> for T {}

// pub trait LayoutHandler<T: LayoutType, L = <T as LayoutType>::L> {
//     fn _layout_impl(&self) -> MemoryLayout;
// }

// impl<T> LayoutHandler<T, CLayout> for T
// where
//     T: LayoutType<L = CLayout>,
// {
//     fn _layout_impl(&self) -> MemoryLayout {
//         MemoryLayout::C
//     }
// }

// impl<T> LayoutHandler<T, FortranLayout> for T
// where
//     T: LayoutType<L = FortranLayout>,
// {
//     fn _layout_impl(&self) -> MemoryLayout {
//         MemoryLayout::F
//     }
// }

// impl<T> LayoutHandler<T, CustomLayout> for T
// where
//     T: LayoutType<L = CustomLayout>,
// {
//     fn _layout_impl(&self) -> MemoryLayout {
//         MemoryLayout::CUSTOM
//     }
// }

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
pub trait MatrixTrait<
    'a,
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
>:
    RandomAccess<Item>
    + Dimensions
    + LayoutType<L = Layout>
    + SizeType<S = Size>
    + Iterable<'a, Item, Iter>
{
}

/// Combined trait for mutable matrices
pub trait MatrixMutTrait<
    'a,
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
    IterMut: Iterator<Item = &'a mut Item>
>:
    RandomAccess<Item>
    + Dimensions
    + LayoutType<L = Layout>
    + SizeType<S = Size>
    + Iterable<'a, Item, Iter>
    + IterableMut<'a, Item, IterMut>
{
}

// Implement `MatrixTrait` for any eligible object.
impl<'a, Item, Layout, Size, Mat, Iter> MatrixTrait<'a, Item, Layout, Size, Iter> for Mat
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
    Mat: RandomAccess<Item>
        + Dimensions
        + LayoutType<L = Layout>
        + SizeType<S = Size>
        + Iterable<'a, Item, Iter>,
{
}

// Implement `MatrixTraitMut` for any eligible object.
impl<'a, Item, Layout, Size, Mat, Iter, IterMut> MatrixMutTrait<'a, Item, Layout, Size, Iter, IterMut> for Mat
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Iter: Iterator<Item = Item>,
    IterMut: Iterator<Item = &'a mut Item>,
    Mat: RandomAccessMut<Item>
        + Dimensions
        + LayoutType<L = Layout>
        + SizeType<S = Size>
        + Iterable<'a, Item, Iter>
        + IterableMut<'a, Item, IterMut>,
{
}
