
use crate::common::*;


pub struct Delaunay<const D: usize>
{
    vertices: Vec<Vector<D>>,
}


impl<const D: usize> Delaunay<D>
{
    pub fn new(vertices: Vec<Vector<D>>) -> Self
    {
        Delaunay {
            vertices,
        }
    }
}