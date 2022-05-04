//! Addition of two matrices

use crate::matrix::*;
use crate::traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub struct Addition<'a, Item, Mat1, Mat2, Layout, Size>(
    Mat1,
    Mat2,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<Size>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>;

impl<'a, Item, Mat1, Mat2, Layout, Size> Addition<'a, Item, Mat1, Mat2, Layout, Size>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    pub fn new(mat1: Mat1, mat2: Mat2) -> Self {
        assert_eq!(
            mat1.dim(),
            mat2.dim(),
            "Dimensions not identical in a + b with a.dim() = {:#?}, b.dim() = {:#?}",
            mat1.dim(),
            mat2.dim()
        );
        Self(
            mat1,
            mat2,
            PhantomData,
            PhantomData,
            PhantomData,
            PhantomData,
        )
    }
}

impl<'a, Item, Mat1, Mat2, Layout, Size> Dimensions for Addition<'a, Item, Mat1, Mat2, Layout, Size>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, Mat1, Mat2, Layout, Size> SafeRandomAccess
    for Addition<'a, Item, Mat1, Mat2, Layout, Size>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    #[inline]
    fn get(&self, row: usize, col: usize) -> Self::Output {
        self.0.get(row, col) + self.1.get(row, col)
    }
    #[inline]
    fn get1d(&self, index: usize) -> Self::Output {
        self.0.get1d(index) + self.1.get1d(index)
    }
}

impl<'a, Item, Mat1, Mat2, Layout, Size> UnsafeRandomAccess
    for Addition<'a, Item, Mat1, Mat2, Layout, Size>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Item;

    #[inline]
    unsafe fn get_unchecked(&self, row: usize, col: usize) -> Self::Output {
        self.0.get_unchecked(row, col) + self.1.get_unchecked(row, col)
    }
    #[inline]
    unsafe fn get1d_unchecked(&self, index: usize) -> Self::Output {
        self.0.get1d_unchecked(index) + self.1.get1d_unchecked(index)
    }
}

impl<'a, Item, Mat1, Mat2, Layout, Size> SizeType for Addition<'a, Item, Mat1, Mat2, Layout, Size>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type S = Size;
}

impl<'a, Item, Mat1, Mat2, Layout, Size> LayoutType<Layout> for Addition<'a, Item, Mat1, Mat2, Layout, Size>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, Size>,
    Mat2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{}

//mat1 + mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, Size>
    std::ops::Add<Matrix<'a, Item, MatImpl2, Layout, Size>>
    for Matrix<'a, Item, MatImpl1, Layout, Size>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, Size>,
    MatImpl2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            Matrix<'a, Item, MatImpl1, Layout, Size>,
            Matrix<'a, Item, MatImpl2, Layout, Size>,
            Layout,
            Size,
        >,
        Layout,
        Size,
    >;

    fn add(self, rhs: Matrix<'a, Item, MatImpl2, Layout, Size>) -> Self::Output {
        Matrix::new(Addition::new(self, rhs))
    }
}

//mat1 + &mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, Size>
    std::ops::Add<&'a Matrix<'a, Item, MatImpl2, Layout, Size>>
    for Matrix<'a, Item, MatImpl1, Layout, Size>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, Size>,
    MatImpl2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            Matrix<'a, Item, MatImpl1, Layout, Size>,
            MatrixFromRef<'a, Item, MatImpl2, Layout, Size>,
            Layout,
            Size,
        >,
        Layout,
        Size,
    >;

    fn add(self, rhs: &'a Matrix<'a, Item, MatImpl2, Layout, Size>) -> Self::Output {
        Matrix::new(Addition::new(self, Matrix::from_ref(rhs)))
    }
}

//&mat1 + mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, Size>
    std::ops::Add<Matrix<'a, Item, MatImpl2, Layout, Size>>
    for &'a Matrix<'a, Item, MatImpl1, Layout, Size>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, Size>,
    MatImpl2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            MatrixFromRef<'a, Item, MatImpl1, Layout, Size>,
            Matrix<'a, Item, MatImpl2, Layout, Size>,
            Layout,
            Size,
        >,
        Layout,
        Size,
    >;

    fn add(self, rhs: Matrix<'a, Item, MatImpl2, Layout, Size>) -> Self::Output {
        Matrix::new(Addition::new(Matrix::from_ref(self), rhs))
    }
}

//&mat1 + &mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, Size>
    std::ops::Add<&'a Matrix<'a, Item, MatImpl2, Layout, Size>>
    for &'a Matrix<'a, Item, MatImpl1, Layout, Size>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, Size>,
    MatImpl2: MatrixTrait<'a, Item, Layout, Size>,
    Layout: LayoutIdentifier,
    Size: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            MatrixFromRef<'a, Item, MatImpl1, Layout, Size>,
            MatrixFromRef<'a, Item, MatImpl2, Layout, Size>,
            Layout,
            Size,
        >,
        Layout,
        Size,
    >;

    fn add(self, rhs: &'a Matrix<'a, Item, MatImpl2, Layout, Size>) -> Self::Output {
        Matrix::new(Addition::new(Matrix::from_ref(self), Matrix::from_ref(rhs)))
    }
}
