//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::boxing::ABoxable;
use crate::common::{Vec3, Vector};
use crate::geometry::{Curve, Line, BCURVE_DER_MAX};
use crate::utilities::normalize_min_max;
use crate::viewer::common::{CurveColor, Convert, Viewable};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use topohedral_viewer::{Color, d3::Client3D, d2::Client2D, d2, d3};
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------


/// Options to use when adding a mesh representing a line to the viewer
#[derive(Debug)]
pub struct LineViewOptions<const D: usize>
{
    /// First distance along the line to start the line segment, must be greater than `dist1`
    pub dist1: f64,
    /// Second distance along the line to start the line segment, must be less than `dist2`
    pub dist2: f64,
    /// Color in which to render the line
    pub color: CurveColor<D>,
}

impl Viewable for Line<2>
{
    type Options = LineViewOptions<2>;
    fn view(
        &mut self,
        port: usize,
        options: &LineViewOptions<2>,
    )
    {
        //{{{ trace
        info!("Viewing line onn port {} with options {:?}", port, options);
        //}}}
        let p1 = self.eval(options.dist1);
        let p2 = self.eval(options.dist2);
        let line_color = match options.color
        {
            CurveColor::Solid(color) => color,
            _ => Color::default(),
        };

        let line_disc = d2::LineDescriptor{
            v1: p1.convert(),
            v2: p2.convert(),
            color: line_color,
        };

        match Client2D::new(port) {
            Ok(mut client) => {
                match client.add_line(line_disc) {
                    Ok(plane_id) => {
                        //{{{ trace
                        info!("Plane added with id: {}", plane_id);
                        //}}}
                    }
                    Err(e) => {
                        //{{{ trace
                        error!("Failed to add plane: {}", e);
                        //}}}
                    }
                }
            }
            Err(e) => {
                //{{{ trace
                error!("Failed to connect to client: {}", e);
                //}}}
            }
        }
    }
}

impl Viewable for Line<3>
{
    type Options = LineViewOptions<3>;
    fn view(
        &mut self,
        port: usize,
        options: &LineViewOptions<3>,
    )
    {
        //{{{ trace
        info!("Viewing line onn port {} with options {:?}", port, options);
        //}}}
        let p1 = self.eval(options.dist1);
        let p2 = self.eval(options.dist2);

        let line_color = match options.color
        {
            CurveColor::Solid(color) => color,
            _ => Color::default(),
        };

        let line_disc = d3::LineDescriptor {
            v1: p1.convert(), 
            v2: p2.convert(), 
            color: line_color
        };

        match Client3D::new(port) {
            Ok(mut client) => {
                match client.add_line(line_disc){
                    Ok(plane_id) => {
                        //{{{ trace
                        info!("Plane added with id: {}", plane_id);
                        //}}}
                    }
                    Err(e) => {
                        //{{{ trace
                        error!("Failed to add plane: {}", e);
                        //}}}
                    }
                }
            }
            Err(e) => {
                //{{{ trace
                error!("Failed to connect to client: {}", e);
                //}}}
            }
        }
    }
}
