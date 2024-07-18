use topohedral_viewer::{run_minimal_2d, run_minimal_3d};

// re-exports 
pub use topohedral_viewer::{Color, d2, d3};
//..................................................................................................
// core 
mod common;
pub use crate::viewer::common::Viewable3D;
pub use common::{CurveColor, CurveViewMethod, SurfaceColor};
//..................................................................................................
// misc
mod view_box;
pub use view_box::{ABoxViewOptions};
//..................................................................................................
// curves
mod view_line;
mod view_bcurve;
pub use view_line::{LineViewOptions};
pub use view_bcurve::{BcurveViewOptions, CtrlPointOptions};
//..................................................................................................
// surfaces
mod view_plane;
pub use view_plane::{PlaneViewOptions};
//..................................................................................................

pub fn run_viewer_2d(state: d2::State<'static>)
{
    pollster::block_on(run_minimal_2d(state));
}

pub fn run_viewer_3d(state: d3::State<'static>)
{
    pollster::block_on(run_minimal_3d(state));
}
