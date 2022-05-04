//! Useful macros

/// Generate a new matrix with C Layout
#[macro_export]
macro_rules! mat {

        ($ScalarType:ty, $dim:expr, $Layout:ty) =>

    {
        crate::matrix::Matrix::<
            '_,
            $ScalarType,
            crate::base_types::DynamicMatrix<$ScalarType, $Layout>,
            $Layout,
            crate::traits::MatrixD,
        >::from_dimension($dim.0, $dim.1)
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! col_vec {

    ($ScalarType:ty, $len:expr) =>

    {
        crate::matrix::Matrix::<
            '_,
            $ScalarType,
            crate::base_types::DynamicMatrix<$ScalarType, VLayout>,
            VLayout,
            crate::traits::MatrixD,
        >::from_dimension($len, 1)
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! row_vec {

    ($ScalarType:ty, $len:expr) =>

    {
        crate::matrix::Matrix::<
            '_,
            $ScalarType,
            crate::base_types::DynamicMatrix<$ScalarType, VLayout>,
            VLayout,
            crate::traits::MatrixD,
        >::from_dimension(1, $len)
    };
}



#[cfg(test)]
mod test {

    use crate::traits::*;

    #[test]
    fn create_c_layout_matrix() {
        let dim = (2, 3);
        let mat = mat![f64, dim, CLayout];

        assert_eq!(mat.dim(), (2, 3));
        assert_eq!(mat.layout_type(), MemoryLayout::C);

    }

    #[test]
    fn create_f_layout_matrix(){
        let dim = (2, 3);
        let mat = mat![f64, dim, FLayout];

        assert_eq!(mat.dim(), (2, 3));
        assert_eq!(mat.layout_type(), MemoryLayout::F);
    }

    #[test]
    fn create_column_vector() {
        let len = 5;
        let vec = col_vec![f64, len];

        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn create_row_vector() {
        let len = 5;
        let vec = row_vec![f64, len];

        assert_eq!(vec.len(), 5);
    }


}
