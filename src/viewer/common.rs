



use topohedral_viewer::{Color, d2, d3};

use crate::common::{Vec2, Vec3};
pub use topohedral_viewer::Vec3 as Vec3f32;

pub trait Vecf64ToVecf32 {
    fn convert(self) -> Vec3f32;
}

impl Vecf64ToVecf32 for Vec2 {
    fn convert(self) -> Vec3f32 {
        Vec3f32::new(self.x as f32, self.y as f32, 0.0f32)
    }
}

impl Vecf64ToVecf32 for Vec3 {
    fn convert(self) -> Vec3f32 {
        Vec3f32::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// Options for generating points on a curve
pub enum CurveViewMethod
{
    /// Points uniformly spaced in parameter space
    Uniform,
    /// Points clustered in areas of high curvature
    Curvature,
}

impl Default for CurveViewMethod
{
    fn default() -> Self
    {
        CurveViewMethod::Uniform
    }
}
//..................................................................................................

pub enum CurveColor
{
    Solid(Color),
    ParamFunction(Box<dyn Fn(f64) -> f64>),
    PositionFunction(Box<dyn Fn(Vec3) -> f64>),
}

impl Default for CurveColor
{
    fn default() -> Self
    {
        CurveColor::Solid(Color::default())
    }
}
//..................................................................................................

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
pub trait Viewable2D {

    /// This type, which must be implemented on a case-by-case basis for each type, provides the 
    /// visualisation options for that type.
    type Options;

    /// This method converts the type into a mesh representation which can be rendered by the viewer
    /// 
    /// It does so by: 
    /// - Creating a mesh representation of the object, there can be many such meshes
    /// - Adding each mesh to the state object so that it can be rendered
    fn view(&mut self, state: &mut d2::State, opts: &Self::Options);
}   

/// Any type which implements this trait can be viewed in the viewer
/// 
/// Any implementation of this trait should provide a way to convert the type into a mesh 
/// representation which for graphical viewing which can be rendered by the viewer
pub trait Viewable3D {

    /// This type, which must be implemented on a case-by-case basis for each type, provides the 
    /// visualisation options for that type.
    type Options;

    /// This method converts the type into a mesh representation which can be rendered by the viewer
    /// 
    /// It does so by: 
    /// - Creating a mesh representation of the object, there can be many such meshes
    /// - Adding each mesh to the state object so that it can be rendered
    fn view(&mut self, state: &mut d3::State, opts: &Self::Options);
}   