//! Useful macros

/// Generate a new matrix with C Layout
#[macro_export]
macro_rules! mat {
    ($ScalarType:ty, $dim:expr, CLayout) => {
        crate::matrix::Matrix::<
            '_,
            $ScalarType,
            crate::base_matrix::DynamicMatrixCLayout<$ScalarType>,
            crate::matrix_traits::CLayout,
            crate::matrix_traits::MatrixD,
            crate::iterators::CopiedSliceIterator<'_, $ScalarType>,
        >::from_dimensions($dim.0, $dim.1)
    };

    ($ScalarType:ty, $dim:expr, FortranLayout) => {
        crate::matrix::Matrix::<
            '_,
            $ScalarType,
            crate::base_matrix::DynamicMatrixFortranLayout<$ScalarType>,
            crate::matrix_traits::FortranLayout,
            crate::matrix_traits::MatrixD,
            crate::iterators::CopiedSliceIterator<'_, $ScalarType>,
        >::from_dimensions_f($dim.0, $dim.1)
    };
}

#[cfg(test)]
mod test {

    use crate::matrix_traits::*;

    #[test]
    fn create_c_layout_matrix() {
        let dim = (2, 3);
        let mat = mat![f64, dim, FortranLayout];

        assert_eq!(mat.dim(), (2, 3));
    }
}
