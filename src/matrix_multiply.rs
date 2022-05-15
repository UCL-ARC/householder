//! Implementation of matrix multiplication

use crate::matrix::*;
use crate::traits::*;
use cauchy::{c32, c64, Scalar};
use matrixmultiply::{cgemm, dgemm, sgemm, zgemm, CGemmOption};
use num;

pub trait Dot<Rhs> {
    type Output;

    fn dot(&self, rhs: &Rhs) -> Self::Output;
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

macro_rules! mat_mat_dot_impl_real {
    ($Scalar:ty, $Blas:ident, $MatType:ident) => {
        impl<'a, 'b> Dot<$MatType<'a, $Scalar>> for $MatType<'b, $Scalar> {
            type Output = $MatType<'static, $Scalar>;
            /// Return the product of this matrix with another matrix.
            fn dot(&self, rhs: &$MatType<'a, $Scalar>) -> Self::Output {
                let dim1 = self.dim();
                let dim2 = rhs.dim();

                assert_eq!(
                    dim1.1, dim2.0,
                    "Matrix multiply incompatible dimensions: A = {:#?}, B = {:#?}",
                    dim1, dim2
                );

                let m = dim1.0;
                let k = dim1.1;
                let n = dim2.1;

                let mut res = $MatType::<$Scalar>::from_dimension(m, n);

                let rsa = self.row_stride() as isize;
                let csa = self.column_stride() as isize;
                let rsb = rhs.row_stride() as isize;
                let csb = rhs.column_stride() as isize;
                let rsc = res.row_stride() as isize;
                let csc = res.column_stride() as isize;

                unsafe {
                    $Blas(
                        m,
                        k,
                        n,
                        num::cast::<f64, $Scalar>(1.0).unwrap(),
                        self.as_ptr(),
                        rsa,
                        csa,
                        rhs.as_ptr(),
                        rsb,
                        csb,
                        num::cast::<f64, $Scalar>(0.0).unwrap(),
                        res.as_mut_ptr(),
                        rsc,
                        csc,
                    );
                }

                res
            }
        }
    };
}


macro_rules! mat_mat_dot_impl_complex {
    ($Scalar:ty, $Real:ty, $Blas:ident, $MatType:ident) => {
        impl<'a, 'b> Dot<$MatType<'a, $Scalar>> for $MatType<'b, $Scalar> {
            type Output = $MatType<'static, $Scalar>;
            /// Return the product of this matrix with another matrix.
            fn dot(&self, rhs: &$MatType<'a, $Scalar>) -> Self::Output {
                let dim1 = self.dim();
                let dim2 = rhs.dim();

                assert_eq!(
                    dim1.1, dim2.0,
                    "Matrix multiply incompatible dimensions: A = {:#?}, B = {:#?}",
                    dim1, dim2
                );

                let m = dim1.0;
                let k = dim1.1;
                let n = dim2.1;

                let mut res = $MatType::<$Scalar>::from_dimension(m, n);

                let rsa = self.row_stride() as isize;
                let csa = self.column_stride() as isize;
                let rsb = rhs.row_stride() as isize;
                let csb = rhs.column_stride() as isize;
                let rsc = res.row_stride() as isize;
                let csc = res.column_stride() as isize;

                let one: [$Real; 2] = [1.0, 0.0];
                let zero: [$Real; 2] = [0.0, 0.0];

                unsafe {
                    $Blas(
                        CGemmOption::Standard,
                        CGemmOption::Standard,
                        m,
                        k,
                        n,
                        one,
                        self.as_ptr() as *const [$Real; 2],
                        rsa,
                        csa,
                        rhs.as_ptr() as *const [$Real; 2],
                        rsb,
                        csb,
                        zero,
                        res.as_mut_ptr() as *mut [$Real; 2],
                        rsc,
                        csc,
                    );
                }

                res
            }
        }

        };
    }


mat_mat_dot_impl_real!(f32, sgemm, FMatrixD);
mat_mat_dot_impl_real!(f32, sgemm, CMatrixD);
mat_mat_dot_impl_real!(f64, dgemm, CMatrixD);    
mat_mat_dot_impl_real!(f64, dgemm, FMatrixD);    
mat_mat_dot_impl_complex!(c32, f32, cgemm, CMatrixD);
mat_mat_dot_impl_complex!(c32, f32, cgemm, FMatrixD);
mat_mat_dot_impl_complex!(c64, f64, zgemm, CMatrixD);
mat_mat_dot_impl_complex!(c64, f64, zgemm, FMatrixD);

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;
    use crate::mat;

    #[test]
    fn dot_product_real_double_c() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![f64, dim1, CLayout];
        let mut mat2 = mat![f64, dim2, CLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = count as f64;
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = count as f64;
                count += 1;
            }
        }

        let mut expected = mat![f64, (dim1.0, dim2.1), CLayout];

        *expected.get_mut(0, 0) = 38.0;
        *expected.get_mut(0, 1) = 41.0;
        *expected.get_mut(0, 2) = 44.0;
        *expected.get_mut(0, 3) = 47.0;
        *expected.get_mut(1, 0) = 128.0;
        *expected.get_mut(1, 1) = 140.0;
        *expected.get_mut(1, 2) = 152.0;
        *expected.get_mut(1, 3) = 164.0;

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col), expected.get(row, col));
            }
        }
    }

    #[test]
    fn dot_product_real_double_f() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![f64, dim1, FLayout];
        let mut mat2 = mat![f64, dim2, FLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = count as f64;
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = count as f64;
                count += 1;
            }
        }

        let mut expected = mat![f64, (dim1.0, dim2.1), CLayout];

        *expected.get_mut(0, 0) = 38.0;
        *expected.get_mut(0, 1) = 41.0;
        *expected.get_mut(0, 2) = 44.0;
        *expected.get_mut(0, 3) = 47.0;
        *expected.get_mut(1, 0) = 128.0;
        *expected.get_mut(1, 1) = 140.0;
        *expected.get_mut(1, 2) = 152.0;
        *expected.get_mut(1, 3) = 164.0;

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col), expected.get(row, col));
            }
        }
    }

    #[test]
    fn dot_product_real_single_c() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![f32, dim1, CLayout];
        let mut mat2 = mat![f32, dim2, CLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = count as f32;
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = count as f32;
                count += 1;
            }
        }

        let mut expected = mat![f32, (dim1.0, dim2.1), CLayout];

        *expected.get_mut(0, 0) = 38.0;
        *expected.get_mut(0, 1) = 41.0;
        *expected.get_mut(0, 2) = 44.0;
        *expected.get_mut(0, 3) = 47.0;
        *expected.get_mut(1, 0) = 128.0;
        *expected.get_mut(1, 1) = 140.0;
        *expected.get_mut(1, 2) = 152.0;
        *expected.get_mut(1, 3) = 164.0;

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col), expected.get(row, col));
            }
        }
    }

    #[test]
    fn dot_product_real_single_f() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![f32, dim1, FLayout];
        let mut mat2 = mat![f32, dim2, FLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = count as f32;
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = count as f32;
                count += 1;
            }
        }

        let mut expected = mat![f32, (dim1.0, dim2.1), CLayout];

        *expected.get_mut(0, 0) = 38.0;
        *expected.get_mut(0, 1) = 41.0;
        *expected.get_mut(0, 2) = 44.0;
        *expected.get_mut(0, 3) = 47.0;
        *expected.get_mut(1, 0) = 128.0;
        *expected.get_mut(1, 1) = 140.0;
        *expected.get_mut(1, 2) = 152.0;
        *expected.get_mut(1, 3) = 164.0;

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col), expected.get(row, col));
            }
        }
    }

    #[test]
    fn dot_product_complex_double_c() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![c64, dim1, CLayout];
        let mut mat2 = mat![c64, dim2, CLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = c64::new(1.0, 1.0) * c64::new(count as f64, 0.0);
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = c64::new(1.0, 2.0) * c64::new(count as f64, 0.0);
                count += 1;
            }
        }

        let mut expected = mat![c64, (dim1.0, dim2.1), CLayout];

        *expected.get_mut(0, 0) = c64::new(-38.0, 114.0);
        *expected.get_mut(0, 1) = c64::new(-41.0, 123.0);
        *expected.get_mut(0, 2) = c64::new(-44.0, 132.0);
        *expected.get_mut(0, 3) = c64::new(-47.0, 141.0);
        *expected.get_mut(1, 0) = c64::new(-128.0, 384.0);
        *expected.get_mut(1, 1) = c64::new(-140.0, 420.0);
        *expected.get_mut(1, 2) = c64::new(-152.0, 456.0);
        *expected.get_mut(1, 3) = c64::new(-164.0, 492.0);

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
            }
        }
    }

    #[test]
    fn dot_product_complex_double_f() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![c64, dim1, FLayout];
        let mut mat2 = mat![c64, dim2, FLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = c64::new(1.0, 1.0) * c64::new(count as f64, 0.0);
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = c64::new(1.0, 2.0) * c64::new(count as f64, 0.0);
                count += 1;
            }
        }

        let mut expected = mat![c64, (dim1.0, dim2.1), FLayout];

        *expected.get_mut(0, 0) = c64::new(-38.0, 114.0);
        *expected.get_mut(0, 1) = c64::new(-41.0, 123.0);
        *expected.get_mut(0, 2) = c64::new(-44.0, 132.0);
        *expected.get_mut(0, 3) = c64::new(-47.0, 141.0);
        *expected.get_mut(1, 0) = c64::new(-128.0, 384.0);
        *expected.get_mut(1, 1) = c64::new(-140.0, 420.0);
        *expected.get_mut(1, 2) = c64::new(-152.0, 456.0);
        *expected.get_mut(1, 3) = c64::new(-164.0, 492.0);

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
            }
        }
    }

    #[test]
    fn dot_product_complex_single_c() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![c32, dim1, CLayout];
        let mut mat2 = mat![c32, dim2, CLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = c32::new(1.0, 1.0) * c32::new(count as f32, 0.0);
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = c32::new(1.0, 2.0) * c32::new(count as f32, 0.0);
                count += 1;
            }
        }

        let mut expected = mat![c32, (dim1.0, dim2.1), CLayout];

        *expected.get_mut(0, 0) = c32::new(-38.0, 114.0);
        *expected.get_mut(0, 1) = c32::new(-41.0, 123.0);
        *expected.get_mut(0, 2) = c32::new(-44.0, 132.0);
        *expected.get_mut(0, 3) = c32::new(-47.0, 141.0);
        *expected.get_mut(1, 0) = c32::new(-128.0, 384.0);
        *expected.get_mut(1, 1) = c32::new(-140.0, 420.0);
        *expected.get_mut(1, 2) = c32::new(-152.0, 456.0);
        *expected.get_mut(1, 3) = c32::new(-164.0, 492.0);

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
            }
        }
    }

    #[test]
    fn dot_product_complex_single_f() {
        let dim1 = (2, 3);
        let dim2 = (3, 4);

        let mut mat1 = mat![c32, dim1, FLayout];
        let mut mat2 = mat![c32, dim2, FLayout];

        let mut count = 0;
        for row in 0..dim1.0 {
            for col in 0..dim1.1 {
                *mat1.get_mut(row, col) = c32::new(1.0, 1.0) * c32::new(count as f32, 0.0);
                count += 1;
            }
        }

        for row in 0..dim2.0 {
            for col in 0..dim2.1 {
                *mat2.get_mut(row, col) = c32::new(1.0, 2.0) * c32::new(count as f32, 0.0);
                count += 1;
            }
        }

        let mut expected = mat![c32, (dim1.0, dim2.1), FLayout];

        *expected.get_mut(0, 1) = c32::new(-41.0, 123.0);
        *expected.get_mut(0, 0) = c32::new(-38.0, 114.0);
        *expected.get_mut(0, 2) = c32::new(-44.0, 132.0);
        *expected.get_mut(0, 3) = c32::new(-47.0, 141.0);
        *expected.get_mut(1, 0) = c32::new(-128.0, 384.0);
        *expected.get_mut(1, 1) = c32::new(-140.0, 420.0);
        *expected.get_mut(1, 2) = c32::new(-152.0, 456.0);
        *expected.get_mut(1, 3) = c32::new(-164.0, 492.0);

        let actual = mat1.dot(&mat2);

        for row in 0..dim1.0 {
            for col in 0..dim2.1 {
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
                assert_relative_eq!(actual.get(row, col).re, expected.get(row, col).re);
            }
        }
    }
}
