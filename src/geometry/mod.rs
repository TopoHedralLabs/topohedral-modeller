//! This is the geometry module, all geometric entities are defined here.
//! 
//! 
//! 
//! 

// Misc
mod common;
// .................................................................................................
// Curves
mod curve;

pub use common::Curve;
pub use curve::line::{Line, LineDescriptor};
pub use curve::bcurve::{Bcurve, BcurveDescriptor, BCURVE_DER_MAX};
// .................................................................................................
// Surfaces
mod surface;

pub use common::Surface;
pub use surface::plane::{Plane, PlaneDescriptor};
pub use surface::bsurface::{Bsurface, BsurfaceDescriptor, BSURFACE_DER_MAX};
// .................................................................................................
