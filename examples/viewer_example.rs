#![feature(generic_const_exprs)]

use topohedral_modeller::boxing::{ABox, ABoxable};
use topohedral_modeller::common::Vec3;
use topohedral_modeller::geometry::{Bcurve, BcurveDescriptor, Curve, Line, LineDescriptor, Plane, PlaneDescriptor};
#[cfg(feature = "viewer")]
use topohedral_modeller::viewer::{
    ABoxViewOptions, BcurveViewOptions, Color, CtrlPointOptions, CurveColor,
    CurveViewMethod, d3, Viewable3D, LineViewOptions, PlaneViewOptions, SurfaceColor
};

fn line_view()
{
    #[cfg(feature = "viewer")]
    {
        let mut line = Line::new(&LineDescriptor {
            origin: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 1.0).normalize(),
        });

        let line_opts = LineViewOptions {
            dist1: -5.0,
            dist2: 5.0,
            color: CurveColor::Solid(Color::Red),
        };

        line.view(50051, &line_opts);
    }
}

fn bcurve_view()
{
    #[cfg(feature = "viewer")]
    {

        let mut bcurve = Bcurve::new(&BcurveDescriptor {
            p: 2,
            knots: vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0],
            cpoints: vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(2.0, 1.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(5.0, 0.0, 1.0),
                Vec3::new(5.0, 0.0, 2.0),
            ],
            cweights: vec![1.0, 3.0, 1.0, 2.0, 1.0, 1.0],
        });

        let kappa = bcurve.curvature_fn();

        let bcurve_opts = BcurveViewOptions {
            method: CurveViewMethod::Uniform,
            num_div: 100,
            color: CurveColor::ParamFunction(Box::new(kappa)),
            with_param_pts: true,
            with_ctrl_pts: CtrlPointOptions::NoPts,
        };

        bcurve.view(50051, &bcurve_opts);

        let abox_opts = ABoxViewOptions {
            color: Color::White,
        };
        let mut abox = bcurve.get_box().clone();
        abox.view(50051, &abox_opts);
    }
}


fn plane_view()
{

    #[cfg(feature = "viewer")]
    {
        let mut state = d3::State::new();

        let mut plane = Plane::new(&PlaneDescriptor{
            origin: Vec3::zeros(), 
            x: Vec3::x(), 
            y: Vec3::y(),
        });

        let plane_opts = PlaneViewOptions {
            x_min: -5.0,
            x_max: 10.0, 
            y_min: 1.0, 
            y_max: 2.0, 
            color: SurfaceColor::Solid(Color::Blue),
        };
        plane.view(50051, &plane_opts);
    }
}

fn main()
{
    // line_view();
    bcurve_view()
    // plane_view()
}
