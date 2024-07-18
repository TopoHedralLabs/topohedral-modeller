//! # Topohedral Modeller
//! ## Introduction
//! Topohedral is a collection of crates which together implement a 3D modelling kernel.

#![feature(generic_const_exprs)]
#![feature(is_sorted)]
#![feature(float_next_up_down)]


//---------------------------------------- Docs ------------------------------------------------- //


#[cfg(test)] mod test_utils;
mod splines;
mod utilities;
pub mod spatial;
pub mod mesh;
pub mod boxing;
pub mod common;
pub mod geometry;
pub mod topology;   
#[cfg(feature = "viewer")] pub mod viewer;



