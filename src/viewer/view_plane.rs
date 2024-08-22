
use std::thread::panicking;

use topohedral_viewer::{
    d3::{CuboidDescriptor, LineDescriptor, PlaneDescriptor, Mesh, State, State3D, Vertex, VertexDescriptor},
    Color, Colormap, ColormapError, CellType
};

use crate::boxing::ABoxable;
use crate::common::{Vec3, Vector};
use crate::geometry::{Plane};
use crate::utilities::normalize_min_max;
use crate::viewer::common::{Vec3f32, Vecf64ToVecf32, Viewable3D, CurveColor, SurfaceColor};


pub struct PlaneViewOptions {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub color: SurfaceColor,
}

impl Viewable3D for Plane
{
    type Options = PlaneViewOptions;

    fn view(&mut self, port: usize, opts: &Self::Options) {

        let plane_disc = PlaneDescriptor {
            origin: self.origin().convert(),
            x_axis: self.x().convert(),
            y_axis: self.y().convert(),
            x_min: opts.x_min as f32,
            x_max: opts.x_max as f32,
            y_min: opts.y_min as f32,
            y_max: opts.y_max as f32,
            line_color: match opts.color {
                SurfaceColor::Solid(color) => color,
                _ => Color::default(),
            }, 
            tri_color:match opts.color {
                SurfaceColor::Solid(color) => color,
                _ => Color::default(),
            }, 
            cell_type: CellType::Triangle
        };

        // state.add_plane(&plane_disc);
    }
}