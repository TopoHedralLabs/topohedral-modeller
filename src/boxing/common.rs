//! This submodule contains the common types, functions, constants, enums and traits which are 
//! shared across this module
//!
//--------------------------------------------------------------------------------------------------


#[derive(Debug, Clone)]
pub struct ABox
{
    dims: [f64; 6]
}
//..................................................................................................

impl ABox
{
    pub fn new_2d(x1: f64, x2: f64, y1: f64, y2: f64) -> Self {

        ABox {
            dims: [x1, x2, y1, y2, f64::NAN, f64::NAN]
        }
    }
    pub fn new_3d(x1: f64, x2: f64, y1: f64, y2: f64, z1: f64, z2: f64) -> Self {
        ABox {
            dims: [x1, x2, y1, y2, z1, z2]
        }
    }
    pub fn xmin(&self) -> f64 { self.dims[0] }
    pub fn xmax(&self) -> f64 { self.dims[1] }
    pub fn ymin(&self) -> f64 { self.dims[2] }
    pub fn ymax(&self) -> f64 { self.dims[3] }
    pub fn zmin(&self) -> f64 
    { 
        if self.is_3d()
        {
            self.dims[4] 
        }
        else {
            0.0
        }
    }
    pub fn zmax(&self) -> f64 
    { 
        if self.is_3d()
        {
            self.dims[5]
        }
        else {
            0.0
        }
    }
    pub fn is_2d(&self) -> bool {
        self.dims[4].is_nan() && self.dims[5].is_nan()
    }
    pub fn is_3d(&self) -> bool {
        !self.dims[4].is_nan() && !self.dims[5].is_nan()
    }

    pub fn diameter(&self) -> f64 {
        let dx = self.xmax() - self.xmin();
        let dy = self.ymax() - self.ymin();
        let dz = self.zmax() - self.zmin();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

}
//..................................................................................................

impl From<[f64; 4]> for ABox {
    fn from(arr: [f64; 4]) -> Self {
        ABox::new_2d(arr[0], arr[1], arr[2], arr[3])
    }
}   
//..................................................................................................

impl From<[f64; 6]> for ABox {
    fn from(arr: [f64; 6]) -> Self {
        ABox::new_3d(arr[0], arr[1], arr[2], arr[3], arr[4], arr[5])
    }
}
//..................................................................................................

/// This trait defines boxable types. Meaning types with a presence in 2D or 3D space for which 
/// the limits of their extent can be computed and stored in a bounding box.
/// 
/// Types that implement this trait are expected to have a ``Option<Box>`` field that is 
/// lazily evaluated
pub trait ABoxable {
    fn compute_box(&mut self);
    fn get_box(&mut self) -> &ABox;
}