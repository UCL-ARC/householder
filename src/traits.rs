//! Basic traits for matrices
pub mod random_access;
pub mod size;
pub mod layout;
pub mod matrix;

pub use random_access::*;
pub use size::*;
pub use layout::*;
pub use matrix::*;



// /// Implement this trait to provide an iterator
// pub trait Iterable<'a, Item: Scalar, Iter: Iterator<Item = Item>> {
//     fn iter(&'a self) -> Iter;
// }

// // Implement this trait to provide a mutable
// pub trait IterableMut<'a, Item: Scalar, Iter: Iterator<Item = &'a mut Item>> {
//     fn iter_mut(&'a mut self) -> Iter;
// }


// /// Access to a raw pointer containing the data
// pub trait Pointer {
//     type Item: Scalar;

//     fn as_ptr(&self) -> *const Self::Item;
// }

// /// Return a slice from data
// pub trait Slice<'a> {
//     type Item: Scalar;

//     fn as_slice(&'a self) -> &'a [Self::Item];
// }

// /// Return a mutable slice from data
// pub trait SliceMut<'a> {
//     type Item: Scalar;

//     fn as_slice_mut(&'a mut self) -> &'a mut [Self::Item];
// }

// impl<'a, Item: Scalar, T: Pointer<Item = Item> + Dimensions> Slice<'a> for T {
//     type Item = Item;

//     fn as_slice(&'a self) -> &'a [Self::Item] {
//         let dim = self.dim();
//         unsafe { std::slice::from_raw_parts(self.as_ptr(), dim.0 * dim.1) }
//     }
// }

// impl<'a, Item: Scalar, T: PointerMut<Item = Item> + Dimensions> SliceMut<'a> for T {
//     type Item = Item;

//     fn as_slice_mut(&'a mut self) -> &'a mut [Self::Item] {
//         let dim = self.dim();
//         unsafe { std::slice::from_raw_parts_mut(self.as_mut_ptr(), dim.0 * dim.1) }
//     }
// }

// /// Mutable access to a raw pointer containing the data
// pub trait PointerMut {
//     type Item: Scalar;

//     fn as_mut_ptr(&mut self) -> *mut Self::Item;
// }

// // Implement `MatrixTrait` for any eligible object.
// impl<'a, Item, Layout, RS, CS, Mat> MatrixTrait<'a, Item, Layout, RS, CS> for Mat
// where
//     Item: Scalar,
//     Layout: LayoutIdentifier,
//     RS: SizeIdentifier,
//     CS: SizeIdentifier,
//     Mat: RandomAccess<Item> + Dimensions + LayoutType<Layout> + SizeType<R = RS, C = CS>,
// {
// }

// // Implement `MatrixTraitMut` for any eligible object.
// impl<'a, Item, Layout, RS, CS, Mat> MatrixMutTrait<'a, Item, Layout, RS, CS> for Mat
// where
//     Item: Scalar,
//     Layout: LayoutIdentifier,
//     RS: SizeIdentifier,
//     CS: SizeIdentifier,
//     Mat: RandomAccessMut<Item> + Dimensions + LayoutType<Layout> + SizeType<R = RS, C = CS>,
// {
// }

// /// Length of a vector
// pub trait VectorLength {
//     fn len(&self) -> usize;
// }

// pub trait Stride {
//     fn row_stride(&self) -> usize;
//     fn column_stride(&self) -> usize;
// }
