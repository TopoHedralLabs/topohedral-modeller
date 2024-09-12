//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::boxing::ABoxable;
use crate::common::{Vec3, Vector};
use crate::geometry::{Bcurve, Curve, BCURVE_DER_MAX};
use crate::viewer::common::{tv, Convert, Viewable, CurveViewMethod, CurveColor};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use topohedral_viewer::{CellType, Color, d2, d3, d2::Mesh2D, d3::Mesh3D};
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ enum: CtrlPointOptions
pub enum CtrlPointOptions
{
    NoPts,
    WithPts(Color),
}
//}}}
//{{{ impl Default for CtrlPointOptions
impl Default for CtrlPointOptions
{
    fn default() -> Self
    {
        CtrlPointOptions::NoPts
    }
}
//..................................................................................................
//}}}
//{{{ struct: BcurveViewOptions
/// Options for Viewing a B-spline curve
#[derive(Default)]
pub struct BcurveViewOptions<const D: usize>
{
    /// Method to use
    pub method: CurveViewMethod,
    /// Number of divisions in Viewed curve
    pub num_div: usize,
    /// Color options for the curve
    pub color: CurveColor<D>,
    /// Show parameter points
    pub with_param_pts: bool,
    /// Controls whether to include the control points in the visualization
    pub with_ctrl_pts: CtrlPointOptions,
}
//..................................................................................................
//}}}
//{{{ collection: 2D Viewing 
//{{{ impl: Bcurve<2>
impl Bcurve<2>
{

    /// This method renders the B-curve with greater sampling density on areas of high curvature.
    fn view_curvature(
        &mut self,
        port: usize,
        opts: &BcurveViewOptions<2>,
    )
    {
        todo!()
    }

    /// This method renders the B-curve with an even distribution of sample points in parameter 
    /// space
    fn view_uniform(
        &mut self,
        port: usize,
        opts: &BcurveViewOptions<2>,
    )
    {
        let nl = opts.num_div;    
        let np = nl + 1;
        let u1  = *self.knots().first().unwrap();
        let u2 = *self.knots().last().unwrap();
        let du = (u2 - u1) / nl as f64; 

        let mut mesh = d2::Mesh::from_num_lines(nl);
        for i in 0..np
        {
            let u = u1 + i as f64 * du;
            let p = self.eval(u);
            mesh.add_vertex(&p.convert(), &Color::default(), &Color::default())
        }

        for i in 0..nl
        {
            mesh.add_line_indices(i as u32, (i+1) as u32);
        }
    }
}
//}}}
//{{{ impl: Viewable for Bcurve<2>
impl Viewable for Bcurve<2>
{
    type Options = BcurveViewOptions<2>;

    fn view(
        &mut self,
        port: usize,
        opts: &Self::Options,
    )
    {
        match opts.method
        {
            CurveViewMethod::Uniform => self.view_uniform(port, opts),
            CurveViewMethod::Curvature => self.view_curvature(port, opts),
        };
    }
}
//}}}
//}}}
//{{{ collection: 3D Viewing 
//{{{ impl: Bcurve<3>
impl Bcurve<3>
{

    /// This method renders the B-curve with greater sampling density on areas of high curvature.
    fn view_curvature(
        &mut self,
        port: usize,
        opts: &BcurveViewOptions<3>,
    )
    {
        todo!()
    }

    /// This method renders the B-curve with an even distribution of sample points in parameter 
    /// space
    fn view_uniform(
        &mut self,
        port: usize,
        opts: &BcurveViewOptions<3>,
    )
    {
        let nl = opts.num_div;    
        let np = nl + 1;
        let u1  = *self.knots().first().unwrap();
        let u2 = *self.knots().last().unwrap();
        let du = (u2 - u1) / nl as f64; 
        let normal = tv::Vec3::zeros();

        let mut mesh = d3::Mesh::from_num_lines(nl);
        for i in 0..np
        {
            let u = u1 + i as f64 * du;
            let p = self.eval(u);

            let color = match opts.color {
                CurveColor::Solid(c) => c,
                _ => Color::default(),

            };
            mesh.add_vertex(&p.convert(), &normal, &color, &color)
        }

        for i in 0..nl
        {
            mesh.add_line_indices(i as u32, (i+1) as u32).unwrap();
        }

        match d3::Client3D::new(port) {
            Ok(mut client) => {
                match client.add_mesh(mesh){
                    Ok(mesh_id) => {
                        //{{{ trace
                        info!("Plane added with id: {}", mesh_id);
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
//}}}
//{{{ impl: Viewable for Bcurve<2>
impl Viewable for Bcurve<3>
{
    type Options = BcurveViewOptions<3>;

    fn view(
        &mut self,
        port: usize,
        opts: &Self::Options,
    )
    {
        match opts.method
        {
            CurveViewMethod::Uniform => self.view_uniform(port, opts),
            CurveViewMethod::Curvature => self.view_curvature(port, opts),
        };
    }
}
//}}}
//}}}
