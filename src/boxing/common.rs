//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::Vector;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: ABox<const D: usize> 
#[derive(Debug, Clone)]
pub struct ABox<const D: usize>
{
    min: [f64; D], 
    max: [f64; D],
}
//..................................................................................................
//}}}
//{{{ impl<const D: usize>  ABox<D>
impl<const D: usize>  ABox<D>
{
    //{{{ fun: new
    pub fn new(min: [f64; D], max: [f64; D]) -> Self
    {
        Self {
            min: min,
            max: max,
        }
    }
    //}}}
    //{{{ fun min
    pub fn min(&self, i: usize) -> f64 { self.min[i] }  
    //}}}
    //{{{ fun max
    pub fn max(&self, i: usize) -> f64 { self.max[i] }  
    //}}}
    //{{{ fun length
    pub fn length(&self, i: usize) -> f64{ self.max[i] - self.min[i] }    
    //}}}
    //{{{ fun: diameter
    pub fn diameter(&self) -> f64 {
        let mut diam = 0.0f64;
        for i in 0..D {
            diam += (self.max[i] - self.min[i]).powi(2);
        }
        diam.sqrt() 
    }
    //}}}
    //{{{ fun: measure
    pub fn measure(&self) -> f64 { 
        let mut meas = 1.0f64;
        for i in 0..D {
            meas *= (self.max[i] - self.min[i]);
        }
        meas
    }
    //}}}
    //{{{ fun: origin
    pub fn origin(&self) -> Vector<D> {
        let mut origin = Vector::<D>::zeros();
        origin.copy_from_slice(&self.min);
        origin
    }
    //}}}
    //{{{ fun: center
    fn center(&self) -> Vector<D>
    {
        let mut center = Vector::<D>::zeros();
        for i in 0..D {
            center[i] = (self.max[i] + self.min[i]) / 2.0;
        }
        center
    }
    //}}}
}
//..................................................................................................
//}}}
//{{{ impl ABox<2>
impl ABox<2>
{

    pub fn xmin(&self) -> f64 { self.min[0] }
    pub fn xmax(&self) -> f64 { self.max[0] }
    pub fn ymin(&self) -> f64 { self.min[1] }
    pub fn ymax(&self) -> f64 { self.max[1] }
}
//}}}
//{{{ impl ABox<3>
impl ABox<3> 
{

    pub fn xmin(&self) -> f64 { self.min[0] }
    pub fn xmax(&self) -> f64 { self.max[0] }
    pub fn ymin(&self) -> f64 { self.min[1] }
    pub fn ymax(&self) -> f64 { self.max[1] }
    pub fn zmin(&self) -> f64 { self.min[2] }
    pub fn zmax(&self) -> f64 { self.max[2] }
}
//}}}
//{{{ impl ABoxable
/// This trait defines boxable types. Meaning types with a presence in 2D or 3D space for which 
/// the limits of their extent can be computed and stored in a bounding box.
/// 
/// Types that implement this trait are expected to have a ``Option<Box>`` field that is 
/// lazily evaluated
pub trait ABoxable<const D: usize> {
    fn get_box(&mut self) -> &ABox<D>;
}//}}}
