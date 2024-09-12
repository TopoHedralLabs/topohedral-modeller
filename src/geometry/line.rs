//! This module contains the definition of the Line curve
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{vec_unitary, Descriptor, Vector};
use super::{common::Surface, Curve};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct LineDescriptor
pub struct LineDescriptor<const D: usize>
{
    pub origin: Vector<D>, 
    pub dir: Vector<D>,
}
//}}}
//{{{ impl: Descriptor for LineDescriptor<D> 
impl<const D: usize> Descriptor for LineDescriptor<D> {
    fn is_valid(&self) -> Result<(), crate::common::DescriptorError> {
        if !vec_unitary(&self.dir, -1.0)
        {
            return Err(crate::common::DescriptorError::InvalidInput(
                "direction vector not unitary".to_string(),
            ));
        }
        Ok(())
    }
}
//}}}
//{{{ struct: Line
pub struct Line<const D: usize>
{
    origin: Vector<D>, 
    dir: Vector<D>,
}
//}}}
//{{{ impl: Line<D>
impl<const D: usize> Line<D>
{
    pub fn new(ld: &LineDescriptor<D>) -> Self {
        debug_assert!(ld.is_valid().is_ok());   
        Line {
            origin: ld.origin,
            dir: ld.dir,
        }
    }   
}
//}}}
//{{{ impl Curve for Line<D>
impl<const D: usize> Curve for Line<D>   
{
    //{{{ type Vector
    type Vector = Vector<D>;
    //}}}
    //{{{ fun: eval
    fn eval(
        &self,
        u: f64,
    ) -> Self::Vector {
        self.origin + u * self.dir  
    }
    //}}}
    //{{{ fun: eval_diff
    fn eval_diff(
        &self,
        u: f64,
        m: usize,
    ) -> Self::Vector {
        match m 
        {
            0 => self.eval(u),
            1 => self.dir,
            _ => Vector::<D>::zeros(),
        }
    }
    //}}}
    //{{{ fun: eval_diff_all
    fn eval_diff_all(
        &self,
        u: f64,
        m: usize,
        ders: &mut [Self::Vector],
    ) {
        debug_assert!(ders.len() >= m +1, "Output array is not large enough");

        for i in 0..=m {
            ders[i] = self.eval_diff(u, i);
        }
    }
    //}}}
    //{{{ fun: eval_arclen
    fn eval_arclen(
        &self,
        u1: f64,
        u2: f64,
    ) -> f64 {
        debug_assert!(u2 > u1);
        u2 - u1
    }
    //}}}
    //{{{ fun: is_member
    fn is_member(
        &self,
        u: f64,
    ) -> bool {
        true
    }
    //}}}
    //{{{ fun: dim
    fn dim(&self) -> usize {
        D
    }
    //}}}
    //{{{ fun: max_der
    fn max_der(&self, u: f64) -> usize {
        1
    }
    //}}}
    //{{{ fun: min_value_scalar
    fn min_value_scalar<F: Fn(f64) -> f64>(&self, f: F, param_range: Option<(f64, f64)>) -> (f64, f64) {
        todo!()
    }
    //}}}
    //{{{ fun: min_value_vector
    fn min_value_vector<F: Fn(Self::Vector) -> f64>(&self, f: F, param_range: Option<(f64, f64)>) -> (f64, f64) {
        todo!()
    }
    //}}}
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn line_new_test() {
        let ld = LineDescriptor {
            origin: Vector::<3>::new(1.0, 2.0, 3.0),
            dir: Vector::<3>::new(0.0, 0.0, 1.0),
        };
        let line = Line::new(&ld);
    }
}
//}}}