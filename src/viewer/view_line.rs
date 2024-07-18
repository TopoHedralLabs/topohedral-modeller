use std::thread::panicking;

use topohedral_viewer::{
    d3::{CuboidDescriptor, LineDescriptor, Mesh, State, State3D, Vertex, VertexDescriptor},
    Color, Colormap, ColormapError,
};

use crate::boxing::ABoxable;
use crate::common::{Vec3, Vector};
use crate::geometry::{Curve, Line, BCURVE_DER_MAX};
use crate::utilities::normalize_min_max;
use crate::viewer::common::{CurveColor, Vec3f32, Vecf64ToVecf32, Viewable3D};

/// Options to use when adding a mesh representing a line to the viewer
pub struct LineViewOptions
{
    /// First distance along the line to start the line segment, must be greater than `dist1`
    pub dist1: f64,
    /// Second distance along the line to start the line segment, must be less than `dist2`
    pub dist2: f64,
    /// Color in which to render the line
    pub color: CurveColor,
}

impl<const D: usize> Viewable3D for Line<D>
{
    type Options = LineViewOptions;
    fn view(
        &mut self,
        state: &mut State,
        options: &LineViewOptions,
    )
    {
        let p1 = self.eval(options.dist1);
        let p1_f32 = if D == 2
        {
            Vec3f32::new(p1[0] as f32, p1[1] as f32, 0.0)
        }
        else
        {
            Vec3f32::new(p1[0] as f32, p1[1] as f32, p1[2] as f32)
        };
        let p2 = self.eval(options.dist2);
        let p2_f32 = if D == 2
        {
            Vec3f32::new(p2[0] as f32, p2[1] as f32, 0.0)
        }
        else
        {
            Vec3f32::new(p2[0] as f32, p2[1] as f32, p2[2] as f32)
        };

        let line_color = match options.color
        {
            CurveColor::Solid(color) => color,
            _ => Color::default(),
        };

        state.add_line(&LineDescriptor {
            p1: p1_f32,
            p2: p2_f32,
            color: line_color,
        });
    }
}
