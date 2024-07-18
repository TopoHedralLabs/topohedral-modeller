//! This is the boxing module. It contains the functionality to compute bounding boxes for 
//! geometric and topological objects.
//!
//--------------------------------------------------------------------------------------------------
// misc
mod common;
pub use common::{ABoxable, ABox};
//..................................................................................................
// curves
mod box_bcurve;
//..................................................................................................

