//! Common definitions for the viewer submodule
//!
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Vec2, Vec3, Vector};
//}}}
//{{{ std imports 
use std::fmt::Debug;
//}}}
//{{{ dep imports 
pub use topohedral_viewer as tv;
use topohedral_viewer::Color;
//}}}
//--------------------------------------------------------------------------------------------------

pub trait Convert<const D: usize> 
{
    fn convert(&self) -> tv::VecD<D>;
}

impl Convert<2> for Vec2
{
    fn convert(&self) -> tv::VecD<2>
    {
        tv::VecD::<2>::new(self.x as f32, self.y as f32)
    }
}

impl Convert<3> for Vec3
{
    fn convert(&self) -> tv::VecD<3>
    {
        tv::VecD::<3>::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

//{{{ collection: CurveViewMethod
//{{{ enum: CurveViewMethod
/// Options for generating points on a curve
pub enum CurveViewMethod
{
    /// Points uniformly spaced in parameter space
    Uniform,
    /// Points clustered in areas of high curvature
    Curvature,
}
//}}}
//{{{ impl: Default for CurveViewMethod
impl Default for CurveViewMethod
{
    fn default() -> Self
    {
        CurveViewMethod::Uniform
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: CurveColor
//{{{ enum:  CurveColor
pub enum CurveColor<const D: usize>
{
    None,
    Solid(Color),
    ParamFunction(Box<dyn Fn(f64) -> f64>),
    PositionFunction(Box<dyn Fn(Vector<D>) -> f64>),
}
//}}}
//{{{ impl : Default for CurveColor
impl<const D: usize> Default for CurveColor<D>
{
    fn default() -> Self
    {
        CurveColor::Solid(Color::default())
    }
}
//}}}
//{{{ impl: Debug for CurveColor
impl<const D: usize> Debug for CurveColor<D>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurveColor::None => write!(f, "None"),
            CurveColor::Solid(c) => write!(f, "Solid({:?})", c),
            CurveColor::ParamFunction(_) => write!(f, "ParamFunction"),
            CurveColor::PositionFunction(_) => write!(f, "PositionFunction"),
        }
    }
}
//}}}
//..................................................................................................
//}}}
pub enum SurfaceColor
{
    Solid(Color),
    ParamFunction(Box<dyn Fn(f64, f64) -> f64>),
    PositionFunction(Box<dyn Fn(Vec3) -> f64>),
}

impl Default for SurfaceColor
{
    fn default() -> Self
    {
        SurfaceColor::Solid(Color::default())
    }
}
//..................................................................................................


/// Any type which implements this trait can be viewed in the viewer
/// 
/// Any implementation of this trait should provide a way to convert the type into a mesh 
/// representation which for graphical viewing which can be rendered by the viewer
pub trait Viewable {

    /// This type, which must be implemented on a case-by-case basis for each type, provides the 
    /// visualisation options for that type.
    type Options;

    /// This method converts the type into a mesh representation which can be rendered by the viewer
    /// 
    /// It does so by: 
    /// - Creating a mesh representation of the object, there can be many such meshes
    /// - Sending the mesh to the viewer via grpc which is listening on the given port
    fn view(&mut self, port: usize, opts: &Self::Options);
}   