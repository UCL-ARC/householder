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

pub trait MatMul {
    type Item: Scalar;
    fn matmul<
        L1: LayoutType,
        L2: LayoutType,
        L3: LayoutType,
        Data1: DataContainer<Item = Self::Item>,
        Data2: DataContainer<Item = Self::Item>,
        Data3: DataContainerMut<Item = Self::Item>,
    >(
        alpha: Self::Item,
        mat_a: &Matrix<
            Self::Item,
            BaseMatrix<Self::Item, Data1, L1, Dynamic, Dynamic>,
            L1,
            Dynamic,
            Dynamic,
        >,
        mat_b: &Matrix<
            Self::Item,
            BaseMatrix<Self::Item, Data2, L2, Dynamic, Dynamic>,
            L2,
            Dynamic,
            Dynamic,
        >,
        beta: Self::Item,
        mat_c: &mut Matrix<
            Self::Item,
            BaseMatrix<Self::Item, Data3, L3, Dynamic, Dynamic>,
            L3,
            Dynamic,
            Dynamic,
        >,
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

        impl MatMul for $Scalar {

            type Item = $Scalar;

            fn matmul<
                L1: LayoutType,
                L2: LayoutType,
                L3: LayoutType,
                Data1: DataContainer<Item = $Scalar>,
                Data2: DataContainer<Item = $Scalar>,
                Data3: DataContainerMut<Item = $Scalar>,
            >(
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

        impl MatMul for $Scalar {

            type Item = $Scalar;

            fn matmul<
                L1: LayoutType,
                L2: LayoutType,
                L3: LayoutType,
                Data1: DataContainer<Item = $Scalar>,
                Data2: DataContainer<Item = $Scalar>,
                Data3: DataContainerMut<Item = $Scalar>,
            >(
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
    use approx::{self, assert_relative_eq};

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

    #[test]
    fn test_matmul_f64() {
        let mut mat_a = MatrixD::<f64, RowMajor>::zeros_from_dim(4, 6);
        let mut mat_b = MatrixD::<f64, RowMajor>::zeros_from_dim(6, 5);
        let mut mat_c_actual = MatrixD::<f64, RowMajor>::zeros_from_dim(4, 5);
        let mut mat_c_expect = MatrixD::<f64, RowMajor>::zeros_from_dim(4, 5);

        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        mat_a.fill_from_rand_standard_normal(&mut rng);
        mat_b.fill_from_rand_standard_normal(&mut rng);
        mat_c_actual.fill_from_rand_standard_normal(&mut rng);

        for index in 0..mat_c_actual.layout().number_of_elements() {
            *mat_c_expect.get1d_mut(index) = mat_c_actual.get1d(index);
        }

        let alpha = 1.0;
        let beta = 0.0;

        matmul_expect(alpha, &mat_a, &mat_b, beta, &mut mat_c_expect);
        f64::matmul(alpha, &mat_a, &mat_b, beta, &mut mat_c_actual);

        for index in 0..mat_c_expect.layout().number_of_elements() {
            assert_relative_eq!(mat_c_actual.get1d(index), mat_c_expect.get1d(index));
        }
    }
}

// impl<'a, 'b, Item: Scalar> Dot<ColVectorD<'a, Item>> for CMatrixD<'b, Item> {
//     type Output = ColVectorD<'static, Item>;

//     fn dot(&self, rhs: &ColVectorD<'a, Item>) -> Self::Output {

//         let dim = self.dim();

//         let mut res = ColVectorD<'static, Item>;

//         for row_index in 0..dim.0 {
//             for col_index in 0..dim.1 {

//             }
//         }

//     }

// }

// macro_rules! mat_mat_dot_impl_real {
//     ($Scalar:ty, $Blas:ident, $MatType:ident) => {
//         impl<'a, 'b> Dot<$MatType<'a, $Scalar>> for $MatType<'b, $Scalar> {
//             type Output = $MatType<'static, $Scalar>;
//             /// Return the product of this matrix with another matrix.
//             fn dot(&self, rhs: &$MatType<'a, $Scalar>) -> Self::Output {
//                 let dim1 = self.dim();
//                 let dim2 = rhs.dim();

//                 assert_eq!(
//                     dim1.1, dim2.0,
//                     "Matrix multiply incompatible dimensions: A = {:#?}, B = {:#?}",
//                     dim1, dim2
//                 );

//                 let m = dim1.0;
//                 let k = dim1.1;
//                 let n = dim2.1;

//                 let mut res = $MatType::<$Scalar>::from_dimension(m, n);

//                 let rsa = self.row_stride() as isize;
//                 let csa = self.column_stride() as isize;
//                 let rsb = rhs.row_stride() as isize;
//                 let csb = rhs.column_stride() as isize;
//                 let rsc = res.row_stride() as isize;
//                 let csc = res.column_stride() as isize;

//                 unsafe {
//                     $Blas(
//                         m,
//                         k,
//                         n,
//                         num::cast::<f64, $Scalar>(1.0).unwrap(),
//                         self.as_ptr(),
//                         rsa,
//                         csa,
//                         rhs.as_ptr(),
//                         rsb,
//                         csb,
//                         num::cast::<f64, $Scalar>(0.0).unwrap(),
//                         res.as_mut_ptr(),
//                         rsc,
//                         csc,
//                     );
//                 }

//                 res
//             }
//         }
//     };
// }

// macro_rules! mat_mat_dot_impl_complex {
//     ($Scalar:ty, $Real:ty, $Blas:ident, $MatType:ident) => {
//         impl<'a, 'b> Dot<$MatType<'a, $Scalar>> for $MatType<'b, $Scalar> {
//             type Output = $MatType<'static, $Scalar>;
//             /// Return the product of this matrix with another matrix.
//             fn dot(&self, rhs: &$MatType<'a, $Scalar>) -> Self::Output {
//                 let dim1 = self.dim();
//                 let dim2 = rhs.dim();

//                 assert_eq!(
//                     dim1.1, dim2.0,
//                     "Matrix multiply incompatible dimensions: A = {:#?}, B = {:#?}",
//                     dim1, dim2
//                 );

//                 let m = dim1.0;
//                 let k = dim1.1;
//                 let n = dim2.1;

//                 let mut res = $MatType::<$Scalar>::from_dimension(m, n);

//                 let rsa = self.row_stride() as isize;
//                 let csa = self.column_stride() as isize;
//                 let rsb = rhs.row_stride() as isize;
//                 let csb = rhs.column_stride() as isize;
//                 let rsc = res.row_stride() as isize;
//                 let csc = res.column_stride() as isize;

//                 let one: [$Real; 2] = [1.0, 0.0];
//                 let zero: [$Real; 2] = [0.0, 0.0];

//                 unsafe {
//                     $Blas(
//                         CGemmOption::Standard,
//                         CGemmOption::Standard,
//                         m,
//                         k,
//                         n,
//                         one,
//                         self.as_ptr() as *const [$Real; 2],
//                         rsa,
//                         csa,
//                         rhs.as_ptr() as *const [$Real; 2],
//                         rsb,
//                         csb,
//                         zero,
//                         res.as_mut_ptr() as *mut [$Real; 2],
//                         rsc,
//                         csc,
//                     );
//                 }

//                 res
//             }
//         }

//         };
//     }

// mat_mat_dot_impl_real!(f32, sgemm, FMatrixD);
// mat_mat_dot_impl_real!(f32, sgemm, CMatrixD);
// mat_mat_dot_impl_real!(f64, dgemm, CMatrixD);
// mat_mat_dot_impl_real!(f64, dgemm, FMatrixD);
// mat_mat_dot_impl_complex!(c32, f32, cgemm, CMatrixD);
// mat_mat_dot_impl_complex!(c32, f32, cgemm, FMatrixD);
// mat_mat_dot_impl_complex!(c64, f64, zgemm, CMatrixD);
// mat_mat_dot_impl_complex!(c64, f64, zgemm, FMatrixD);

// #[cfg(test)]
// mod test {
//     use approx::assert_relative_eq;

//     use super::*;
//     use crate::mat;

//     #[test]
//     fn dot_product_real_double_c() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![f64, dim1, CLayout];
//         let mut mat2 = mat![f64, dim2, CLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = count as f64;
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = count as f64;
//                 count += 1;
//             }
//         }

//         let mut expected = mat![f64, (dim1.0, dim2.1), CLayout];

//         *expected.get_mut(0, 0) = 38.0;
//         *expected.get_mut(0, 1) = 41.0;
//         *expected.get_mut(0, 2) = 44.0;
//         *expected.get_mut(0, 3) = 47.0;
//         *expected.get_mut(1, 0) = 128.0;
//         *expected.get_mut(1, 1) = 140.0;
//         *expected.get_mut(1, 2) = 152.0;
//         *expected.get_mut(1, 3) = 164.0;

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col), expected.get(row, col));
//             }
//         }
//     }

//     #[test]
//     fn dot_product_real_double_f() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![f64, dim1, FLayout];
//         let mut mat2 = mat![f64, dim2, FLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = count as f64;
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = count as f64;
//                 count += 1;
//             }
//         }

//         let mut expected = mat![f64, (dim1.0, dim2.1), CLayout];

//         *expected.get_mut(0, 0) = 38.0;
//         *expected.get_mut(0, 1) = 41.0;
//         *expected.get_mut(0, 2) = 44.0;
//         *expected.get_mut(0, 3) = 47.0;
//         *expected.get_mut(1, 0) = 128.0;
//         *expected.get_mut(1, 1) = 140.0;
//         *expected.get_mut(1, 2) = 152.0;
//         *expected.get_mut(1, 3) = 164.0;

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col), expected.get(row, col));
//             }
//         }
//     }

//     #[test]
//     fn dot_product_real_single_c() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![f32, dim1, CLayout];
//         let mut mat2 = mat![f32, dim2, CLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = count as f32;
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = count as f32;
//                 count += 1;
//             }
//         }

//         let mut expected = mat![f32, (dim1.0, dim2.1), CLayout];

//         *expected.get_mut(0, 0) = 38.0;
//         *expected.get_mut(0, 1) = 41.0;
//         *expected.get_mut(0, 2) = 44.0;
//         *expected.get_mut(0, 3) = 47.0;
//         *expected.get_mut(1, 0) = 128.0;
//         *expected.get_mut(1, 1) = 140.0;
//         *expected.get_mut(1, 2) = 152.0;
//         *expected.get_mut(1, 3) = 164.0;

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col), expected.get(row, col));
//             }
//         }
//     }

//     #[test]
//     fn dot_product_real_single_f() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![f32, dim1, FLayout];
//         let mut mat2 = mat![f32, dim2, FLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = count as f32;
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = count as f32;
//                 count += 1;
//             }
//         }

//         let mut expected = mat![f32, (dim1.0, dim2.1), CLayout];

//         *expected.get_mut(0, 0) = 38.0;
//         *expected.get_mut(0, 1) = 41.0;
//         *expected.get_mut(0, 2) = 44.0;
//         *expected.get_mut(0, 3) = 47.0;
//         *expected.get_mut(1, 0) = 128.0;
//         *expected.get_mut(1, 1) = 140.0;
//         *expected.get_mut(1, 2) = 152.0;
//         *expected.get_mut(1, 3) = 164.0;

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col), expected.get(row, col));
//             }
//         }
//     }

//     #[test]
//     fn dot_product_complex_double_c() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![c64, dim1, CLayout];
//         let mut mat2 = mat![c64, dim2, CLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = c64::new(1.0, 1.0) * c64::new(count as f64, 0.0);
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = c64::new(1.0, 2.0) * c64::new(count as f64, 0.0);
//                 count += 1;
//             }
//         }

//         let mut expected = mat![c64, (dim1.0, dim2.1), CLayout];

//         *expected.get_mut(0, 0) = c64::new(-38.0, 114.0);
//         *expected.get_mut(0, 1) = c64::new(-41.0, 123.0);
//         *expected.get_mut(0, 2) = c64::new(-44.0, 132.0);
//         *expected.get_mut(0, 3) = c64::new(-47.0, 141.0);
//         *expected.get_mut(1, 0) = c64::new(-128.0, 384.0);
//         *expected.get_mut(1, 1) = c64::new(-140.0, 420.0);
//         *expected.get_mut(1, 2) = c64::new(-152.0, 456.0);
//         *expected.get_mut(1, 3) = c64::new(-164.0, 492.0);

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//             }
//         }
//     }

//     #[test]
//     fn dot_product_complex_double_f() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![c64, dim1, FLayout];
//         let mut mat2 = mat![c64, dim2, FLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = c64::new(1.0, 1.0) * c64::new(count as f64, 0.0);
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = c64::new(1.0, 2.0) * c64::new(count as f64, 0.0);
//                 count += 1;
//             }
//         }

//         let mut expected = mat![c64, (dim1.0, dim2.1), FLayout];

//         *expected.get_mut(0, 0) = c64::new(-38.0, 114.0);
//         *expected.get_mut(0, 1) = c64::new(-41.0, 123.0);
//         *expected.get_mut(0, 2) = c64::new(-44.0, 132.0);
//         *expected.get_mut(0, 3) = c64::new(-47.0, 141.0);
//         *expected.get_mut(1, 0) = c64::new(-128.0, 384.0);
//         *expected.get_mut(1, 1) = c64::new(-140.0, 420.0);
//         *expected.get_mut(1, 2) = c64::new(-152.0, 456.0);
//         *expected.get_mut(1, 3) = c64::new(-164.0, 492.0);

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//             }
//         }
//     }

//     #[test]
//     fn dot_product_complex_single_c() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![c32, dim1, CLayout];
//         let mut mat2 = mat![c32, dim2, CLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = c32::new(1.0, 1.0) * c32::new(count as f32, 0.0);
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = c32::new(1.0, 2.0) * c32::new(count as f32, 0.0);
//                 count += 1;
//             }
//         }

//         let mut expected = mat![c32, (dim1.0, dim2.1), CLayout];

//         *expected.get_mut(0, 0) = c32::new(-38.0, 114.0);
//         *expected.get_mut(0, 1) = c32::new(-41.0, 123.0);
//         *expected.get_mut(0, 2) = c32::new(-44.0, 132.0);
//         *expected.get_mut(0, 3) = c32::new(-47.0, 141.0);
//         *expected.get_mut(1, 0) = c32::new(-128.0, 384.0);
//         *expected.get_mut(1, 1) = c32::new(-140.0, 420.0);
//         *expected.get_mut(1, 2) = c32::new(-152.0, 456.0);
//         *expected.get_mut(1, 3) = c32::new(-164.0, 492.0);

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//             }
//         }
//     }

//     #[test]
//     fn dot_product_complex_single_f() {
//         let dim1 = (2, 3);
//         let dim2 = (3, 4);

//         let mut mat1 = mat![c32, dim1, FLayout];
//         let mut mat2 = mat![c32, dim2, FLayout];

//         let mut count = 0;
//         for row in 0..dim1.0 {
//             for col in 0..dim1.1 {
//                 *mat1.get_mut(row, col) = c32::new(1.0, 1.0) * c32::new(count as f32, 0.0);
//                 count += 1;
//             }
//         }

//         for row in 0..dim2.0 {
//             for col in 0..dim2.1 {
//                 *mat2.get_mut(row, col) = c32::new(1.0, 2.0) * c32::new(count as f32, 0.0);
//                 count += 1;
//             }
//         }

//         let mut expected = mat![c32, (dim1.0, dim2.1), FLayout];

//         *expected.get_mut(0, 1) = c32::new(-41.0, 123.0);
//         *expected.get_mut(0, 0) = c32::new(-38.0, 114.0);
//         *expected.get_mut(0, 2) = c32::new(-44.0, 132.0);
//         *expected.get_mut(0, 3) = c32::new(-47.0, 141.0);
//         *expected.get_mut(1, 0) = c32::new(-128.0, 384.0);
//         *expected.get_mut(1, 1) = c32::new(-140.0, 420.0);
//         *expected.get_mut(1, 2) = c32::new(-152.0, 456.0);
//         *expected.get_mut(1, 3) = c32::new(-164.0, 492.0);

//         let actual = mat1.dot(&mat2);

//         for row in 0..dim1.0 {
//             for col in 0..dim2.1 {
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//                 assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
//             }
//         }
//     }
// }
