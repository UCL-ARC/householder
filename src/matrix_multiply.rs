//! Implementation of matrix multiplication

use crate::base_matrix::BaseMatrix;
use crate::data_container::{DataContainer, DataContainerMut, VectorContainer};
use crate::layouts::*;
use crate::matrix::*;
use crate::traits::*;
use cauchy::{c32, c64, Scalar};
use matrixmultiply::{cgemm, dgemm, sgemm, zgemm, CGemmOption};
use num;

pub trait Dot<Rhs> {
    type Output;

    fn dot(&self, rhs: &Rhs) -> Self::Output;
}

pub trait MatMul<
    Item: Scalar,
    L1: LayoutType,
    L2: LayoutType,
    L3: LayoutType,
    Data1: DataContainer<Item = Item>,
    Data2: DataContainer<Item = Item>,
    Data3: DataContainerMut<Item = Item>,
    RS1: SizeIdentifier,
    RS2: SizeIdentifier,
    RS3: SizeIdentifier,
    CS1: SizeIdentifier,
    CS2: SizeIdentifier,
    CS3: SizeIdentifier,
>
{
    fn matmul(
        alpha: Item,
        mat_a: &Matrix<Item, BaseMatrix<Item, Data1, L1, RS1, CS1>, L1, RS1, CS1>,
        mat_b: &Matrix<Item, BaseMatrix<Item, Data2, L2, RS2, CS2>, L2, RS2, CS2>,
        beta: Item,
        mat_c: &mut Matrix<Item, BaseMatrix<Item, Data3, L3, RS3, CS3>, L3, RS3, CS3>,
    );
}

macro_rules! dot_impl {
    ($Scalar:ty) => {
        impl<
                L1: LayoutType,
                L2: LayoutType,
                Data1: DataContainer<Item = $Scalar>,
                Data2: DataContainer<Item = $Scalar>,
            >
            Dot<
                Matrix<
                    $Scalar,
                    BaseMatrix<$Scalar, Data2, L2, Dynamic, Dynamic>,
                    L2,
                    Dynamic,
                    Dynamic,
                >,
            >
            for Matrix<
                $Scalar,
                BaseMatrix<$Scalar, Data1, L1, Dynamic, Dynamic>,
                L1,
                Dynamic,
                Dynamic,
            >
        {
            type Output = Matrix<
                $Scalar,
                BaseMatrix<$Scalar, VectorContainer<$Scalar>, RowMajor, Dynamic, Dynamic>,
                RowMajor,
                Dynamic,
                Dynamic,
            >;

            fn dot(
                &self,
                rhs: &Matrix<
                    $Scalar,
                    BaseMatrix<$Scalar, Data2, L2, Dynamic, Dynamic>,
                    L2,
                    Dynamic,
                    Dynamic,
                >,
            ) -> Self::Output {
                let mut res = MatrixD::<$Scalar, RowMajor>::zeros_from_dim(
                    self.layout().dim().0,
                    rhs.layout().dim().1,
                );
                <$Scalar>::matmul(
                    num::cast::<f64, $Scalar>(1.0).unwrap(),
                    &self,
                    rhs,
                    num::cast::<f64, $Scalar>(0.0).unwrap(),
                    &mut res,
                );
                res
            }
        }
    };
}

macro_rules! matmul_real {

    ($Scalar:ty, $Blas:ident) => {

        impl<
        L1: LayoutType,
        L2: LayoutType,
        L3: LayoutType,
        Data1: DataContainer<Item = $Scalar>,
        Data2: DataContainer<Item = $Scalar>,
        Data3: DataContainerMut<Item = $Scalar>
>


        MatMul<
            $Scalar,
            L1,
            L2,
            L3,
            Data1,
            Data2,
            Data3,
            Dynamic,
            Dynamic,
            Dynamic,
            Dynamic,
            Dynamic,
            Dynamic>


        for $Scalar {

            fn matmul(
                alpha: $Scalar,
                mat_a: &Matrix<$Scalar, BaseMatrix<$Scalar, Data1, L1, Dynamic, Dynamic>, L1, Dynamic, Dynamic>,
                mat_b: &Matrix<$Scalar, BaseMatrix<$Scalar, Data2, L2, Dynamic, Dynamic>, L2, Dynamic, Dynamic>,
                beta: $Scalar,
                mat_c: &mut Matrix<$Scalar, BaseMatrix<$Scalar, Data3, L3, Dynamic, Dynamic>, L3, Dynamic, Dynamic>,
            ) {
                let dim1 = mat_a.layout().dim();
                let dim2 = mat_b.layout().dim();
                let dim3 = mat_c.layout().dim();

                assert!(
                    (dim1.1 == dim2.0) & (dim3.0 == dim1.0) & (dim3.1 == dim2.1),
                    "Matrix multiply incompatible dimensions for C = A * B: A = {:#?}, B = {:#?}, C = {:#?}",
                    dim1,
                    dim2,
                    dim3
                );

                let m = dim1.0 as usize;
                let k = dim1.1 as usize;
                let n = dim2.1 as usize;
                let rsa = mat_a.layout().stride().0 as isize;
                let csa = mat_a.layout().stride().1 as isize;
                let rsb = mat_b.layout().stride().0 as isize;
                let csb = mat_b.layout().stride().1 as isize;
                let rsc = mat_c.layout().stride().0 as isize;
                let csc = mat_c.layout().stride().1 as isize;

                unsafe {
                    $Blas(
                        m,
                        k,
                        n,
                        alpha,
                        mat_a.get_pointer(),
                        rsa,
                        csa,
                        mat_b.get_pointer(),
                        rsb,
                        csb,
                        beta,
                        mat_c.get_pointer_mut(),
                        rsc,
                        csc,
                    );
                }
            }

            }

        };

}

macro_rules! matmul_complex {

    ($Scalar:ty, $Real:ty, $Blas:ident) => {

        impl<
        L1: LayoutType,
        L2: LayoutType,
        L3: LayoutType,
        Data1: DataContainer<Item = $Scalar>,
        Data2: DataContainer<Item = $Scalar>,
        Data3: DataContainerMut<Item = $Scalar>
>


        MatMul<
            $Scalar,
            L1,
            L2,
            L3,
            Data1,
            Data2,
            Data3,
            Dynamic,
            Dynamic,
            Dynamic,
            Dynamic,
            Dynamic,
            Dynamic>


        for $Scalar {

            fn matmul(
                alpha: $Scalar,
                mat_a: &Matrix<$Scalar, BaseMatrix<$Scalar, Data1, L1, Dynamic, Dynamic>, L1, Dynamic, Dynamic>,
                mat_b: &Matrix<$Scalar, BaseMatrix<$Scalar, Data2, L2, Dynamic, Dynamic>, L2, Dynamic, Dynamic>,
                beta: $Scalar,
                mat_c: &mut Matrix<$Scalar, BaseMatrix<$Scalar, Data3, L3, Dynamic, Dynamic>, L3, Dynamic, Dynamic>,
            ) {
                let dim1 = mat_a.layout().dim();
                let dim2 = mat_b.layout().dim();
                let dim3 = mat_c.layout().dim();

                assert!(
                    (dim1.1 == dim2.0) & (dim3.0 == dim1.0) & (dim3.1 == dim2.1),
                    "Matrix multiply incompatible dimensions for C = A * B: A = {:#?}, B = {:#?}, C = {:#?}",
                    dim1,
                    dim2,
                    dim3
                );

                let m = dim1.0 as usize;
                let k = dim1.1 as usize;
                let n = dim2.1 as usize;
                let rsa = mat_a.layout().stride().0 as isize;
                let csa = mat_a.layout().stride().1 as isize;
                let rsb = mat_b.layout().stride().0 as isize;
                let csb = mat_b.layout().stride().1 as isize;
                let rsc = mat_c.layout().stride().0 as isize;
                let csc = mat_c.layout().stride().1 as isize;

                let alpha = [alpha.re(), alpha.im()];
                let beta = [beta.re(), beta.im()];

                unsafe {
                    $Blas(
                        CGemmOption::Standard,
                        CGemmOption::Standard,
                        m,
                        k,
                        n,
                        alpha,
                        mat_a.get_pointer() as *const [$Real; 2],
                        rsa,
                        csa,
                        mat_b.get_pointer() as *const [$Real; 2],
                        rsb,
                        csb,
                        beta,
                        mat_c.get_pointer_mut() as *mut [$Real; 2],
                        rsc,
                        csc,
                    );
                }
            }

            }

        };

    }

matmul_real!(f64, dgemm);
matmul_real!(f32, sgemm);
matmul_complex!(c32, f32, cgemm);
matmul_complex!(c64, f64, zgemm);

dot_impl!(f64);
dot_impl!(f32);
dot_impl!(c32);
dot_impl!(c64);

#[cfg(test)]
mod test {

    use super::*;
    use crate::tools::RandScalar;
    use approx::assert_ulps_eq;
    use rand_distr::StandardNormal;

    use rand::prelude::*;

    fn matmul_expect<
        Item: Scalar,
        L1: LayoutType,
        L2: LayoutType,
        L3: LayoutType,
        Data1: DataContainer<Item = Item>,
        Data2: DataContainer<Item = Item>,
        Data3: DataContainerMut<Item = Item>,
    >(
        alpha: Item,
        mat_a: &GenericBaseMatrix<Item, L1, Data1, Dynamic, Dynamic>,
        mat_b: &GenericBaseMatrix<Item, L2, Data2, Dynamic, Dynamic>,
        beta: Item,
        mat_c: &mut GenericBaseMatrix<Item, L3, Data3, Dynamic, Dynamic>,
    ) {
        let m = mat_a.layout().dim().0;
        let k = mat_a.layout().dim().1;
        let n = mat_b.layout().dim().1;

        for m_index in 0..m {
            for n_index in 0..n {
                *mat_c.get_mut(m_index, n_index) *= beta;
                for k_index in 0..k {
                    *mat_c.get_mut(m_index, n_index) +=
                        alpha * mat_a.get(m_index, k_index) * mat_b.get(k_index, n_index);
                }
            }
        }
    }

    macro_rules! matmul_test {
        ($Scalar:ty, $fname:ident) => {
            #[test]
            fn $fname() {
                let mut mat_a = MatrixD::<$Scalar, RowMajor>::zeros_from_dim(4, 6);
                let mut mat_b = MatrixD::<$Scalar, RowMajor>::zeros_from_dim(6, 5);
                let mut mat_c_actual = MatrixD::<$Scalar, RowMajor>::zeros_from_dim(4, 5);
                let mut mat_c_expect = MatrixD::<$Scalar, RowMajor>::zeros_from_dim(4, 5);

                let dist = StandardNormal;

                let mut rng = rand::rngs::StdRng::seed_from_u64(0);

                mat_a.fill_from_rand_standard_normal(&mut rng);
                mat_b.fill_from_rand_standard_normal(&mut rng);
                mat_c_actual.fill_from_rand_standard_normal(&mut rng);

                for index in 0..mat_c_actual.layout().number_of_elements() {
                    *mat_c_expect.get1d_mut(index) = mat_c_actual.get1d(index);
                }

                let alpha = <$Scalar>::random_scalar(&mut rng, &dist);
                let beta = <$Scalar>::random_scalar(&mut rng, &dist);

                matmul_expect(alpha, &mat_a, &mat_b, beta, &mut mat_c_expect);
                <$Scalar>::matmul(alpha, &mat_a, &mat_b, beta, &mut mat_c_actual);

                for index in 0..mat_c_expect.layout().number_of_elements() {
                    let val1 = mat_c_actual.get1d(index);
                    let val2 = mat_c_expect.get1d(index);
                    assert_ulps_eq!(&val1, &val2, max_ulps = 100);
                }
            }
        };
    }
    matmul_test!(f32, test_matmul_f32);
    matmul_test!(f64, test_matmul_f64);
    matmul_test!(c32, test_matmul_c32);
    matmul_test!(c64, test_matmul_c64);
}
