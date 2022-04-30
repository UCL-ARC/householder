//! Definition of iterators

//! For built-in types we use the `std::slice::Iter` struct as implementation.
//! Users can provide custom iterators as well, which are packed within an
//! Iterator struct.

/// The base iterator type
pub type CopiedSliceIterator<'a, Item> = std::iter::Copied<std::slice::Iter<'a, Item>>;

/// The base iterator type for mutable access
pub type SliceIteratorMut<'a, Item> = std::slice::IterMut<'a, Item>;

