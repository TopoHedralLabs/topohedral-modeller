//! This module contains common types, traits and utility functions used throughout the crate.
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
use std::ops::{Add, Index, IndexMut, Mul, Sub};
use std::sync::OnceLock;
//}}}
//{{{ dep imports 
use nalgebra as na;
use thiserror::Error;
use topohedral_integrate::gauss;
//}}}
//--------------------------------------------------------------------------------------------------


//{{{ collection: Vector types
pub type Vector<const N: usize> = na::SVector<f64, N>;
pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;
pub type Vec4 = Vector<4>;
//}}}
//{{{ collection: VectorOps
//{{{ trait: VectorOps
/// This trait provides a set of operations defined for nalgebra vecotrs vectors. This is purely
/// to overcome the limitations of Rusts rules on trait implementations.
pub trait VectorOps:
    Copy + Mul<f64, Output = Self> + Index<usize, Output = f64> + IndexMut<usize>
{
    fn zeros() -> Self;
    fn cross(
        &self,
        other: &Self,
    ) -> Self;
    fn dot(
        &self,
        other: &Self,
    ) -> f64;

    fn norm(&self) -> f64;
    fn normalize(&self) -> Self;
}
//}}}
//{{{ impl: VectorOps for Vector<D>
impl<const D: usize> VectorOps for Vector<D>
{
    fn zeros() -> Self
    {
        Vector::<D>::zeros()
    }

    fn dot(
        &self,
        other: &Self,
    ) -> f64
    {
        self.dot(other)
    }

    fn cross(
        &self,
        other: &Self,
    ) -> Self
    {
        self.cross(other)
    }

    fn norm(&self) -> f64
    {
        self.norm()
    }

    fn normalize(&self) -> Self
    {
        self.normalize()
    }
}
//}}}
//}}}
//{{{ collection: ResConstants 
//{{{ trait: ResConstants
/// Defines set of constants used throughout crate for tolerant floating point comparisons.
pub trait ResConstants
{
    const RES_LINEAR: Self;
    const RES_ANGULAR: Self;
}
//}}}
//{{{ impl: ResConstants for f32
impl ResConstants for f32
{
    const RES_LINEAR: f32 = 1.0e-10;
    const RES_ANGULAR: f32 = 1.0e-8;
}
//}}}
//{{{ impl: ResConstants for f64
impl ResConstants for f64
{
    const RES_LINEAR: f64 = 1.0e-10;
    const RES_ANGULAR: f64 = 1.0e-8;
}
//}}}
//}}}
//{{{ enum: DescriptorError
#[derive(Error, Debug)]
pub enum DescriptorError
{
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
//}}}
//{{{ trait: Descriptor
pub trait Descriptor 
{
    fn is_valid(&self) -> Result<(), DescriptorError>;
}
//}}}
//{{{ fun: vec_equal
/// Determines whether two vectors are equal within a given tolerance.
///
/// # Arguments
/// * `a` - First vector to compare
/// * `b` - Second vector to compare
/// # Returns
/// True if vectors equal within tolerance, false otherwise
fn vec_equal<const D: usize>(
    a: &Vector<D>,
    b: &Vector<D>,
    tol: f64,
) -> bool
{
    (a - b).norm() <= tol
}
//}}}
//{{{ fun: vec_unitary
/// Determines whether a vector is unitary (i.e. has a norm of 1) within a given tolerance.
///
/// # Arguments
/// * `a` - Input vector
/// * `tol` - Tolerance value for determining unitarity
/// # Returns
/// True if unitary within tolerance, false otherwise
pub fn vec_unitary<const D: usize>(
    a: &Vector<D>,
    mut tol: f64,
) -> bool
{
    if tol < 0.0 {
        tol = f64::RES_LINEAR;
    }
    let norm = a.norm();
    (norm - 1.0).abs() <= tol
}
//}}}
//{{{ fun: vec_colinear
///  Determines whether two vectors are colinear
///
/// # Arguments
/// * a - First vector to compare
/// * b - Second vector to compare
///
/// # Returns
/// True if vectors colinear, false otherwise
pub fn vec_colinear<const D: usize> (a: &Vector<D>, b: &Vector<D>, mut tol: f64) -> bool {
    if tol < 0.0 {
        tol = f64::RES_ANGULAR;
    }
    let a_norm = a.norm();
    let b_norm = b.norm();
    let cos_angle = a.dot(b).abs() / (a_norm * b_norm);
    (cos_angle - 1.0).abs() <= tol
}
//}}}
//{{{ fun: vec_orthogonal
/// Determines whether two vectors are orthogonal within a given tolerance.
///
/// # Arguments
/// * `a` - First vector to compare
/// * `b` - Second vector to compare
/// * `tol` - Tolerance value for determining orthogonality
///
/// # Returns
/// True if vectors are orthogonal within tolerance, false otherwise
pub fn vec_orthogonal<const D: usize>(a: &Vector<D>, b: &Vector<D>, mut tol: f64) -> bool 
{
    if tol < 0.0 {
        tol = f64::RES_ANGULAR;
    }
    let a_norm = a.norm();
    let b_norm = b.norm();
    let cos_angle = a.dot(b).abs() / (a_norm * b_norm);
    cos_angle <= tol
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
    
    use std::vec;
    use super::*;

    #[test]
    fn test_vec_equal() {
        let a = Vector::<3>::from_element(1.0);
        let b = Vector::<3>::from_element(1.0);
        let c = Vector::<3>::from_element(1.00001);
        assert!(vec_equal(&a, &b, 1.0e-10));
        assert!(!vec_equal(&a, &c, 1.0e-10));
    }

    #[test]
    fn test_vec_unitary() {
        let a = Vector::<3>::new(1.0, 0.0, 0.0);
        let b = Vector::<3>::new(1.00001, 0.0, 0.0);
        assert!(vec_unitary(&a, 1.0e-10));
        assert!(!vec_unitary(&b, 1.0e-6));
    }

    #[test]
    fn test_vec_colinear() {
        let a = Vector::<3>::new(1.0, 0.0, 0.0);
        let b = Vector::<3>::new(1.0, 0.0, 0.0);
        let c = Vector::<3>::new(1.0, 1.0, 0.0);
        assert!(vec_colinear(&a, &b, 1.0e-10));
        assert!(!vec_colinear(&a, &c, 1.0e-10));    
    }

    #[test]
    fn test_vec_orthogonal() {
        let a = Vector::<3>::new(1.0, 0.0, 0.0);
        let b = Vector::<3>::new(0.0, 1.0, 0.0);
        let c = Vector::<3>::new(1.0, 1.0, 0.0);
        assert!(vec_orthogonal(&a, &b, 1.0e-10));
        assert!(!vec_orthogonal(&a, &c, 1.0e-10));
    }

}
//}}}