//! The purpose of this example is to demonstrate how to render items in the modeller in the 
//! viewer.
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
use std::process::Command;
//}}}
//{{{ dep imports 
use topohedral_modeller::boxing::{ABox, ABoxable};
use topohedral_modeller::common::Vec3;
use topohedral_modeller::geometry::{Bcurve, BcurveDescriptor, Curve, Line, LineDescriptor, Plane, PlaneDescriptor};
#[cfg(feature = "viewer")]
use topohedral_modeller::viewer::{
    ABoxViewOptions, BcurveViewOptions, Color, CtrlPointOptions, CurveColor,
    CurveViewMethod, d3, Viewable, LineViewOptions, PlaneViewOptions, SurfaceColor, tv
};
use topohedral_tracing::*;
#[cfg(feature = "viewer")]
use topohedral_viewer::app::locate_executable;
//}}}
//--------------------------------------------------------------------------------------------------

fn axes_view()
{
    #[cfg(feature = "viewer")]
    {
        //{{{ trace
        info!("Creating the axes");
        //}}}
        let mut client = d3::Client3D::new(50051).unwrap();
        //{{{ trace
        info!("Adding axes");
        //}}}
        let axes_id = client
            .add_axes(d3::AxesDescriptor {
                origin: tv::Vec3::new(0.0, 0.0, 0.0),
                x_axis: tv::Vec3::new(1.0, 0.0, 0.0),
                y_axis: tv::Vec3::new(0.0, 1.0, 0.0),
                z_axis: tv::Vec3::new(0.0, 0.0, 1.0),
                neg_len: 1000.0,
                pos_len: 1000.0,
            })
            .unwrap();
    }
}

fn line_view()
{
    #[cfg(feature = "viewer")]
    {
        //{{{ trace
        info!("Creating the line");
        //}}}
        let mut line = Line::new(&LineDescriptor {
            origin: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 2.0, 3.0).normalize(),
        });

        //{{{ trace
        info!("Setting the view options");
        //}}}
        let line_opts = LineViewOptions {
            dist1: -5.0,
            dist2: 5.0,
            color: CurveColor::Solid(Color::Red),
        };

        //{{{ trace
        info!("Submitting for rendering");
        //}}}
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
            color: CurveColor::Solid(Color::Red),
            // color: CurveColor::ParamFunction(Box::new(kappa)),
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
        let mut plane = Plane::new(&PlaneDescriptor{
            origin: Vec3::new(1.0, 1.0, 1.0), 
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
    init().unwrap();

    let topoviewer_exec_result = locate_executable();
    if let Ok(topoviewer_exec) = topoviewer_exec_result {
        //{{{ trace
        info!("Found topoviewer executable at {:?}", topoviewer_exec);
        //}}}
        let _server_process = Command::new(topoviewer_exec)
            .arg("d3")
            .arg("with-port")
            .arg("50051")
            .spawn()
            .expect("Failed to start topoviewer");

        //{{{ trace
        info!("Sleeping for 2 seconds");
        //}}}
        std::thread::sleep(std::time::Duration::from_millis(2000));

        axes_view();
        //{{{ trace
        info!("Server process running");
        //}}}
        line_view();
        //{{{ trace
        info!("Adding a bcurve");
        //}}}
        bcurve_view();
        //{{{ trace
        info!("Adding a plane");
        //}}}
        plane_view();


        // std::thread::sleep(std::time::Duration::from_millis(10000));

    }
    // bcurve_view()
    // plane_view()
}
