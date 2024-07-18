
use topohedral_viewer::{Color, d3::{State3D,CuboidDescriptor, Mesh, State, Vertex, VertexDescriptor}, Colormap, ColormapError};

use crate::boxing::ABoxable;
use crate::common::{Vec3, Vector};
use crate::geometry::{Bcurve, Curve, BCURVE_DER_MAX};
use crate::utilities::normalize_min_max;
use crate::viewer::common::{Vec3f32, Vecf64ToVecf32, Viewable3D, CurveViewMethod, CurveColor};




pub enum CtrlPointOptions
{
    NoPts,
    WithPts(Color),
}

impl Default for CtrlPointOptions
{
    fn default() -> Self
    {
        CtrlPointOptions::NoPts
    }
}
//..................................................................................................

/// Options for Viewing a B-spline curve
#[derive(Default)]
pub struct BcurveViewOptions
{
    /// Method to use
    pub method: CurveViewMethod,
    /// Number of divisions in Viewed curve
    pub num_div: usize,
    /// Color options for the curve
    pub color: CurveColor,
    /// Show parameter points
    pub with_param_pts: bool,
    /// Controls whether to include the control points in the visualization
    pub with_ctrl_pts: CtrlPointOptions,
}
//..................................................................................................

impl<const D: usize> Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
    [(); D * 2]:,
{

    /// This method renders the B-curve with greater sampling density on areas of high curvature.
    fn view_curvature(
        &mut self,
        state: &mut State,
        opts: &BcurveViewOptions,
    )
    {
        todo!()
    }

    /// This method renders the B-curve with an even distribution of sample points in parameter 
    /// space
    fn view_uniform(
        &mut self,
        state: &mut State,
        opts: &BcurveViewOptions,
    )
    {
        // .......... init
        // Calculate the box of the curve to get a diameter
        let abox = self.get_box();
        let abox_diam = abox.diameter();
        // Get the curve segement data, no. of points etc
        let nl = opts.num_div;
        let np = opts.num_div + 1;
        // Initialise the mesh
        let mut mesh = Mesh::from_num_lines(nl);
        // get the first param and the param delta
        let mut u = *self.knots().first().unwrap();
        let du = 1.0 / (nl as f64);
        // This is not a surface so normal is zero
        let normal = Vec3f32::zeros();

        //.......... com: create the vertices of the curve
        for _ in 0..np
        {
            // evalutate the point
            let point_tmp = self.eval(u);
            // convert to Vec3
            let mut point = Vec3::zeros();
            point[0] = point_tmp[0];
            point[1] = point_tmp[1];
            point[2] =  if D == 3 { point_tmp[2] } else { 0.0};
            // append vertex to set
            mesh.append_vertex(&Vertex::new(&VertexDescriptor {
                position: point.convert(),
                normal: normal,
                line_color: Color::default(),
                triangle_color: Color::default(),
            }));
            // increment the parameter
            u += du;
        }
        //.......... com: add line indices and add mesh to state
        for i in 0..nl
        {
            mesh.append_indices(&[i as u32, (i + 1) as u32])
        }

        let cmesh_uid = state.add_mesh(mesh);
        let mut param_pts_uids = Vec::new();

        //..........  com: add point glyphs to show endpoints of segments
        if opts.with_param_pts {

            u = *self.knots().first().unwrap();

            for _ in 0..np 
            {
                // evalutate the point
                let point_tmp = self.eval(u);
                // convert to Vec3
                let mut point = Vec3::zeros();
                point[0] = point_tmp[0];
                point[1] = point_tmp[1];
                point[2] =  if D == 3 { point_tmp[2] } else { 0.0};
                // determine cube dimensiosn based on diameter of bbox
                let cub_len = abox_diam as f32 / (10.0 * opts.num_div as f32);
                let dx = (cub_len / 2.0) * Vec3f32::new(1.0, 1.0, 1.0);
                let cub_origin = point.convert() - dx;

                let uid = state.add_cuboid(&CuboidDescriptor {
                    origin: cub_origin,
                    x: Vec3f32::x(),
                    y: Vec3f32::y(),
                    z: Vec3f32::z(),
                    lenx: cub_len,
                    leny: cub_len,
                    lenz: cub_len,
                    line_color: Color::default(),
                    tri_color: Color::default(),
                });
                param_pts_uids.push(uid);
                u += du;
            }
        }

        // .......... com add vertex colours
        match &opts.color 
        {
            CurveColor::Solid(c) => {
                let mut cmesh = state.get_mesh_mut(cmesh_uid).unwrap();
                cmesh.set_line_colors(*c);

                for uid in param_pts_uids {
                    let mut cub_mesh = state.get_mesh_mut(cmesh_uid).unwrap();
                    cub_mesh.set_line_colors(*c);
                }
            },
            CurveColor::ParamFunction(f) => {


                u = *self.knots().first().unwrap();

                let mut fvals = Vec::with_capacity(np);
                for _ in 0..np 
                {
                    let fi = f(u);
                    fvals.push(fi as f32);
                    u += du;
                }
                normalize_min_max(&mut fvals);

                let cmesh = state.get_mesh_mut(cmesh_uid).unwrap();
                cmesh.set_line_colors_from_colormap(&fvals, "viridis");

                if opts.with_param_pts {

                    let cmap = Colormap::new("viridis".to_string()).unwrap();
                    for (i, uid) in param_pts_uids.iter().enumerate() {
                        let cub_mesh = state.get_mesh_mut(*uid).unwrap();
                        let c = cmap.get_color(fvals[i]);
                        cub_mesh.set_line_colors(Color::Other((c[0], c[1], c[2])));
                    }
                }
            },
            CurveColor::PositionFunction(f) => {

            },
        }



        if let CtrlPointOptions::WithPts(color) = opts.with_ctrl_pts
        {
            self.view_control_points(state, &color, abox_diam as f32)
        }
    }

    /// This method visualises the control points of the B-curve.
    fn view_control_points(
        &self,
        state: &mut State,
        color: &Color,
        abox_diam: f32,
    )
    {
        let ctrl_pts: Vec<Vector<D>> = self.cpoints();
        let n_ctrl_pts = ctrl_pts.len();
        let mut mesh = Mesh::from_num_lines(n_ctrl_pts - 1);

        for ctrl_pt in ctrl_pts
        {
            let pos: Vec3f32 = if D == 2
            {
                Vec3f32::new(ctrl_pt[0] as f32, ctrl_pt[1] as f32, 0.0)
            }
            else if D == 3
            {
                Vec3f32::new(ctrl_pt[0] as f32, ctrl_pt[1] as f32, ctrl_pt[2] as f32)
            }
            else
            {
                panic!("Unsupported dimension: {}", D);
            };

            let vertex = Vertex::new(&VertexDescriptor {
                position: pos,
                normal: Vec3f32::zeros(),
                line_color: *color,
                triangle_color: *color,
            });
            mesh.append_vertex(&vertex);


            let cub_len = abox_diam as f32 / (30.0 * n_ctrl_pts as f32);
            let dx = (cub_len / 2.0) * Vec3f32::new(1.0, 1.0, 1.0);
            let mut cub_origin = pos - dx;
            state.add_cuboid(&CuboidDescriptor {
                origin: cub_origin,
                x: Vec3f32::x(),
                y: Vec3f32::y(),
                z: Vec3f32::z(),
                lenx: cub_len,
                leny: cub_len,
                lenz: cub_len,
                line_color: *color,
                tri_color: *color,
            });
        }

        for i in 0..n_ctrl_pts - 1
        {
            mesh.append_indices(&[i as u32, (i + 1) as u32])
        }
        state.add_mesh(mesh);
    }
}

impl<const D: usize> Viewable3D for Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
    [(); D * 2]:,
{
    type Options = BcurveViewOptions;

    fn view(
        &mut self,
        state: &mut State,
        opts: &Self::Options,
    )
    {
        match opts.method
        {
            CurveViewMethod::Uniform => self.view_uniform(state, opts),
            CurveViewMethod::Curvature => self.view_curvature(state, opts),
        };
    }
}
