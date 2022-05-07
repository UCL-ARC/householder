//! Addition of two matrices

use crate::matrix::*;
use crate::traits::*;
use cauchy::Scalar;
use std::marker::PhantomData;

pub struct Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>(
    Mat1,
    Mat2,
    PhantomData<Item>,
    PhantomData<Layout>,
    PhantomData<RS>,
    PhantomData<CS>,
    PhantomData<&'a ()>,
)
where
    Item: Scalar,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>;

impl<'a, Item, Mat1, Mat2, Layout, RS, CS> Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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
            PhantomData,
        )
    }
}

impl<'a, Item, Mat1, Mat2, Layout, RS, CS> Dimensions for Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    {
    fn dim(&self) -> (usize, usize) {
        self.0.dim()
    }
}

impl<'a, Item, Mat1, Mat2, Layout, RS, CS> SafeRandomAccess
    for Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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

impl<'a, Item, Mat1, Mat2, Layout, RS, CS> UnsafeRandomAccess
    for Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
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

impl<'a, Item, Mat1, Mat2, Layout, RS, CS> SizeType for Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type R = RS;
    type C = CS;
}

impl<'a, Item, Mat1, Mat2, Layout, RS, CS> LayoutType<Layout> for Addition<'a, Item, Mat1, Mat2, Layout, RS, CS>
where
    Item: Scalar,
    Mat1: MatrixTrait<'a, Item, Layout, RS, CS>,
    Mat2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{}

//mat1 + mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, RS, CS>
    std::ops::Add<Matrix<'a, Item, MatImpl2, Layout, RS, CS>>
    for Matrix<'a, Item, MatImpl1, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, RS, CS>,
    MatImpl2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            Matrix<'a, Item, MatImpl1, Layout, RS, CS>,
            Matrix<'a, Item, MatImpl2, Layout, RS, CS>,
            Layout,
            RS,
            CS,
        >,
        Layout,
        RS,
        CS,
    >;

    fn add(self, rhs: Matrix<'a, Item, MatImpl2, Layout, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(self, rhs))
    }
}

//mat1 + &mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, RS, CS>
    std::ops::Add<&'a Matrix<'a, Item, MatImpl2, Layout, RS, CS>>
    for Matrix<'a, Item, MatImpl1, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, RS, CS>,
    MatImpl2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            Matrix<'a, Item, MatImpl1, Layout, RS, CS>,
            MatrixFromRef<'a, Item, MatImpl2, Layout, RS, CS>,
            Layout,
            RS,
            CS
        >,
        Layout,
        RS,
        CS
    >;

    fn add(self, rhs: &'a Matrix<'a, Item, MatImpl2, Layout, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(self, Matrix::from_ref(rhs)))
    }
}

//&mat1 + mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, RS, CS>
    std::ops::Add<Matrix<'a, Item, MatImpl2, Layout, RS, CS>>
    for &'a Matrix<'a, Item, MatImpl1, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, RS, CS>,
    MatImpl2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            MatrixFromRef<'a, Item, MatImpl1, Layout, RS, CS>,
            Matrix<'a, Item, MatImpl2, Layout, RS, CS>,
            Layout,
            RS,
            CS
        >,
        Layout,
        RS,
        CS,
    >;

    fn add(self, rhs: Matrix<'a, Item, MatImpl2, Layout, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(Matrix::from_ref(self), rhs))
    }
}

//&mat1 + &mat2
impl<'a, Item, MatImpl1, MatImpl2, Layout, RS, CS>
    std::ops::Add<&'a Matrix<'a, Item, MatImpl2, Layout, RS, CS>>
    for &'a Matrix<'a, Item, MatImpl1, Layout, RS, CS>
where
    Item: Scalar,
    MatImpl1: MatrixTrait<'a, Item, Layout, RS, CS>,
    MatImpl2: MatrixTrait<'a, Item, Layout, RS, CS>,
    Layout: LayoutIdentifier,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
{
    type Output = Matrix<
        'a,
        Item,
        Addition<
            'a,
            Item,
            MatrixFromRef<'a, Item, MatImpl1, Layout, RS, CS>,
            MatrixFromRef<'a, Item, MatImpl2, Layout, RS, CS>,
            Layout,
            RS,
            CS,
        >,
        Layout,
        RS,
        CS
    >;

    fn add(self, rhs: &'a Matrix<'a, Item, MatImpl2, Layout, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(Matrix::from_ref(self), Matrix::from_ref(rhs)))
    }
}
