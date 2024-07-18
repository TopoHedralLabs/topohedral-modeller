//! This submodule contains the various flavours of multi-dimensional arrays.
//! 
//! 
use static_assertions::const_assert;
use std::ops::{Index, IndexMut};
use std::fmt::{self, write};
use crate::common::Vector;
const MAX_DIM: usize = 4;


/// This struct provides the index-conversion for going between a 1D index and an N-dimensional index.
/// 
/// Indexing in this struct is implemented such that the leftmost index varies the fastest.
/// In 2D this is equivalent to column-major ordering. So, given a set of dimensions:
/// $$
/// (n_{1}, n_{2}, ..., n_{N})
/// $$
/// The linear index $j$ of a multi-dimensional index $(i_{1}, i_{2}, ..., i_{N})$ is given by:
/// $$
/// j = i_{1} + n_{1} i_{2} + n_{1}n_{2}i_{3} + ... + n_{1}n_{2}...n_{N-1}i_{N}
/// $$
/// And given a linear index $j$, the multi-dimensional index $(i_{1}, i_{2}, ..., i_{N})$ is:
/// $$
/// todo
/// $$
#[repr(C)]
pub struct IndexHelper<const N: usize>
{
    /// Dimensions of the N-dimensional array
    dims: [usize; N],
    /// Helper array for converting a multi-dimensional index into a linear index
    lin_helper: [usize; N],
    /// Helper array for converting a linear index into a multi-dimensional index
    tuple_helper: [usize; N],
}
//..................................................................................................

impl<const N: usize> IndexHelper<N>
{

    pub fn new(dims: &[usize]) -> Self {
        debug_assert!(dims.len() >= N);

        let mut helper = IndexHelper::<N> {
            dims: [0; N],
            lin_helper: [1; N],
            tuple_helper: [0; N],
        };

        helper.dims.clone_from_slice(&dims[0..N]);
        helper.tuple_helper.clone_from_slice(&dims[0..N]);

        for i in 1..N {
            helper.lin_helper[i] = helper.lin_helper[i - 1] * dims[i - 1];
            helper.tuple_helper[i] = helper.tuple_helper[i] * helper.tuple_helper[i - 1];
        }
        helper
    }

    pub fn lin_index(&self, indices: &[usize]) -> usize {
        debug_assert!(indices.len() >= N);
        let mut idx = 0usize;
        for i in 0..N {
            idx += indices[i] * self.lin_helper[i];
        }
        idx
    }
    //..............................................................................................

    pub fn tuple_index(&self, idx: usize) -> [usize; N] {
        let mut out = [0; N];
        out[0] = idx % self.dims[0];
        for i in 1..N {
            let base = (idx / self.tuple_helper[i]) * self.tuple_helper[i];
            out[i] = (idx - base) / self.tuple_helper[i - 1];
        }
        out
    }
    //.............................................................................................
}
//..................................................................................................

/// Provides a wrapper around a pre-existing 1D array to represent a multi-dimensional array.
/// 
/// Indexing is provided by [IndexHelper]
#[repr(C)]
pub struct NDArrayWrapper<'a, T, const N: usize> {
    /// Underlying data
    data: &'a mut [T],
    /// Indexing helper
    idx_helper: IndexHelper<N>,
}
//..................................................................................................

impl<'a, T, const N: usize> NDArrayWrapper<'a, T, N> {
    // const_assert!(N > 0);

    pub fn new(data: &'a mut [T], dims: &[usize]) -> Self {
        debug_assert!(dims.len() >= N);
        debug_assert!(data.len() >= dims.iter().product());

        let nd_array = NDArrayWrapper::<'a, T, N>{
            data: data,
            idx_helper: IndexHelper::new(dims), 
        };

        nd_array
    }
}
//..................................................................................................

impl<'a, T, const N: usize> Index<&[usize; N]> for NDArrayWrapper<'a, T, N>
{
    type Output = T;

    fn index(&self, index_tuple: &[usize; N]) -> &Self::Output {
        let idx = self.idx_helper.lin_index(index_tuple);
        &self.data[idx]
    }
}
//..................................................................................................

impl<'a, T, const N: usize> IndexMut<&[usize; N]> for NDArrayWrapper<'a, T, N>
{
    fn index_mut(&mut self, index_tuple: &[usize; N]) -> &mut Self::Output {
        let idx = self.idx_helper.lin_index(index_tuple);
        &mut self.data[idx]
    }
}
//..................................................................................................

impl<'a, T, const N: usize> fmt::Display for NDArrayWrapper<'a, T, N>
where 
    T: fmt::Display
{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let len = self.idx_helper.dims.iter().product::<usize>();

        match N {
            1 => {
                write!(f, "[")?;
                for i in 0..len-1
                {
                    let val = &self.data[i];
                    write!(f, "{}, ", val)?;
                }
                let val = self.data.last().unwrap();
                write!(f, "{}]", val)?;
            },
            2 => {

                writeln!(f, "[")?;
                for i in 0..self.idx_helper.dims[0]
                {
                    write!(f, " [")?;
                    for j in 0.. self.idx_helper.dims[1]-1
                    {
                        let idx = self.idx_helper.lin_index(&[i, j]);
                        let val = &self.data[idx];
                        write!(f, "{}, ", val)?;
                    }
                    let idx = self.idx_helper.lin_index(&[i, self.idx_helper.dims[1]-1]);
                    let val = &self.data[idx];
                    writeln!(f, "{}]", val)?;
                }
                writeln!(f, "]")?;
            }
            _ if N > 2 => {

                for i in 0..len
                {
                    let i2 = self.idx_helper.tuple_index(i);
                    writeln!(f, "{:?}: {}", i2, self.index(&i2))?;
                }
            },
            _ => {
                panic!("N should not be 0!")
            }
        }

        Ok(())
    }
}
//..................................................................................................



// ------------------------------------------- Tests -------------------------------------------- //
mod tests {
    use crate::utilities::NDArrayWrapper;

    #[test]
    fn linear_index2() {

        let mut data: Vec<f64> = (0..12).map(|n| n as f64).collect();
        let lin_idx = NDArrayWrapper::new(data.as_mut_slice(), &[3, 4]);

        let mut idx1 = 0;
        let mut val1 = 0.0;
        for j in 0..4
        {
            for i in 0..3
            {
                let tuple1 = [i, j];
                let tuple2 = lin_idx.idx_helper.tuple_index(idx1);
                let idx2 = lin_idx.idx_helper.lin_index(&tuple1);
                let val2 = lin_idx[&tuple1];
                assert_eq!(idx1, idx2);
                assert_eq!(tuple1, tuple2);
                assert_eq!(val1, val2);
                idx1 += 1;
                val1 += 1.0;
            }
        }
    }

    #[test]
    fn linear_index3() {

        let mut data: Vec<f64> = (0..24).map(|n| n as f64).collect();
        let lin_idx = NDArrayWrapper::new(data.as_mut_slice(), &[3, 4, 2]);

        let mut idx1 = 0;
        let mut val1 = 0.0;
        for k in 0..2
        {
            for j in 0..4
            {
                for i in 0..3
                {
                    let tuple1 = [i, j, k];
                    let tuple2 = lin_idx.idx_helper.tuple_index(idx1);
                    let idx2 = lin_idx.idx_helper.lin_index(&tuple1);
                    let val2 = lin_idx[&tuple1];
                    assert_eq!(idx1, idx2);
                    assert_eq!(tuple1, tuple2);
                    assert_eq!(val1, val2);
                    idx1 += 1;
                    val1 += 1.0;
                }
            }
        }

    } 



    #[test]
    fn print1() {
        let mut data: Vec<f64> = (0..24).map(|n| n as f64).collect();
        let lin_idx: NDArrayWrapper<'_, f64, 1> = NDArrayWrapper::new(data.as_mut_slice(), &[24]);
        eprintln!("{}", lin_idx);
    }

    #[test]
    fn print2() {
        let mut data: Vec<f64> = (0..24).map(|n| n as f64).collect();
        let lin_idx: NDArrayWrapper<'_, f64, 2> = NDArrayWrapper::new(data.as_mut_slice(), &[4, 6]);
        eprintln!("{}", lin_idx);
    }

    #[test]
    fn print3() {
        let mut data: Vec<f64> = (0..24).map(|n| n as f64).collect();
        let lin_idx: NDArrayWrapper<'_, f64, 3> = NDArrayWrapper::new(data.as_mut_slice(), &[3, 4, 2]);
        eprintln!("{}", lin_idx);
    }

}