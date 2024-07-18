//! This is the geometry module, all geometric entities are defined here.
//! 
//! 
//! 
//! 

// Misc
mod common;
// .................................................................................................
// Curves
mod line;
mod bcurve;

pub use common::Curve;
pub use line::{Line, LineDescriptor};
pub use bcurve::{Bcurve, BcurveDescriptor, BCURVE_DER_MAX};
// .................................................................................................
// Surfaces
mod plane;
mod bsurface;

pub use common::Surface;
pub use plane::{Plane, PlaneDescriptor};
pub use bsurface::{Bsurface, BsurfaceDescriptor, BSURFACE_DER_MAX};
// .................................................................................................
