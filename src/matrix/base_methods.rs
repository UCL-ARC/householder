//! Methods on Matrix types that require an underlying base matrix.

use crate::matrix::Matrix;
use crate::traits::*;
use crate::types::*;
use crate::base_matrix::*;
use crate::data_container::{DataContainerMut};
use rand::prelude::*;
use rand_distr::StandardNormal;

use super::GenericBaseMatrixMut;

impl<Item: Scalar,
     L: LayoutType,
     RS: SizeIdentifier,
     CS: SizeIdentifier,
     Data: DataContainerMut<Item=Item>>
     GenericBaseMatrixMut<Item, L, Data, RS, CS>{

          pub fn for_each<F: FnMut(&mut Item)>(&mut self, mut f: F) {
               for index in 0..self.layout().number_of_elements() {
                    unsafe {f(self.get1d_unchecked_mut(index))}
               }
          }
          
     }

