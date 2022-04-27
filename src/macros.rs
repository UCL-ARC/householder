//! Useful macros

/// Generate a new matrix with C Layout
#[macro_export]
macro_rules! mat {

    ($ScalarType:ty, $dim:expr) => {
        crate::matrix::Matrix::<$ScalarType, crate::base_matrix::BaseMatrixCLayout<$ScalarType>>::from_dimensions($dim.0, $dim.1)
    };
}

#[cfg(test)]
mod test {

    use crate::matrix_traits::*;

    #[test]
    fn create_c_layout_matrix() {
        let dim = (2, 3);
        let mat = mat![f64, dim];

        assert_eq!(mat.dim(), (2, 3));
    }
}
