//! This module provides a miscellaneous set of utilities which are used throughout the crate
//!
//!
//!
//!
//--------------------------------------------------------------------------------------------------

use num_traits::Float;


mod ndarray;


pub use ndarray::{NDArrayWrapper, IndexHelper};


use crate::common::ResConstants;

pub fn lower_bound<T: PartialOrd>(slice: &[T], value: T) -> usize {
    slice.binary_search_by(|probe| probe.partial_cmp(&value).unwrap_or(std::cmp::Ordering::Greater))
        .unwrap_or_else(|x| x)
}

pub fn normalize_min_max<T>(fvals: &mut Vec::<T>)
where 
    T: Float + ResConstants 
{
        let mut min_f = *fvals
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let mut max_f = *fvals
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if max_f - min_f > T::RES_LINEAR {
            fvals.iter_mut().for_each(|x| {
                *x = (*x - min_f) / (max_f - min_f);
            });
        }
}