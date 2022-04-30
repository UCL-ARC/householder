//! Definition of operators on matrices
use crate::matrix_traits::*;
use cauchy::Scalar;
use core::marker::PhantomData;

// ------------------------------
// Scalar Multiplication

/// Container for multiplication with a scalar
pub struct ScalarMult<'a, Item, Mat>(Item, &'a Mat)
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions + LayoutType;

impl<'a, Item, Mat> ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions + LayoutType,
{
    /// Create a new instance of Scalar Multiplication
    pub fn new(factor: Item, mat: &'a Mat) -> Self {
        ScalarMult(factor, mat)
    }
}

impl<'a, Item, Mat> SafeRandomAccess for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions + LayoutType,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0 * self.1.get(row, col)
    }
}

impl<'a, Item, Mat> UnsafeRandomAccess for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions + LayoutType,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0 * self.1.get_unchecked(row, col)
    }
}

impl<'a, Item, Mat> Dimensions for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions + LayoutType,
{
    #[inline]
    fn dim(&self) -> (usize, usize) {
        self.1.dim()
    }
}

impl<'a, Item, Mat, Layout> LayoutType for ScalarMult<'a, Item, Mat>
where
    Item: Scalar,
    Mat: RandomAccess<Item> + Dimensions + LayoutType<L=Layout>,
    Layout: LayoutIdentifier {
        type L = Layout;
    }


// ----------------------------
// Addition

/// Container for multiplication with a scalar
pub struct Add<'a, Item, Mat1, Mat2, Layout>(
    &'a Mat1,
    &'a Mat2,
    PhantomData<Item>,
    PhantomData<Layout>,
)
where
    Item: Scalar,
    Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Layout: LayoutIdentifier;

impl<'a, Item, Mat1, Mat2, Layout> Add<'a, Item, Mat1, Mat2, Layout>
where
    Item: Scalar,
    Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Layout: LayoutIdentifier,
{
    /// Create a new instance of Scalar Multiplication
    pub fn new(mat1: &'a Mat1, mat2: &'a Mat2) -> Self {
        Add(mat1, mat2, PhantomData, PhantomData)
    }
}

impl<'a, Item, Mat1, Mat2, Layout> LayoutType for Add<'a, Item, Mat1, Mat2, Layout>
where
    Item: Scalar,
    Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Layout: LayoutIdentifier,
{
    type L = Layout;
}

impl<'a, Item, Mat1, Mat2, Layout> SafeRandomAccess for Add<'a, Item, Mat1, Mat2, Layout>
where
    Item: Scalar,
    Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Layout: LayoutIdentifier,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col) + self.1.get(row, col)
    }
}

impl<'a, Item, Mat1, Mat2, Layout> UnsafeRandomAccess for Add<'a, Item, Mat1, Mat2, Layout>
where
    Item: Scalar,
    Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Layout: LayoutIdentifier,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0.get_unchecked(row, col) + self.1.get_unchecked(row, col)
    }
}

impl<'a, Item, Mat1, Mat2, Layout> Dimensions for Add<'a, Item, Mat1, Mat2, Layout>
where
    Item: Scalar,
    Mat1: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Mat2: RandomAccess<Item> + Dimensions + LayoutType<L = Layout>,
    Layout: LayoutIdentifier,
{
    #[inline]
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}
