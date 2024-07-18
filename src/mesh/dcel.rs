//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

use crate::common::*;

use std::rc::Rc;
use std::cell::RefCell;


const MUID_NULL: usize = usize::MAX;

pub trait Muid 
{
    fn is_null(&self) -> bool;
    fn null() -> Self;
}

impl Muid for usize {
    fn is_null(&self) -> bool {
        *self == MUID_NULL
    }
    fn null() -> Self {
        MUID_NULL
    }
}
//..................................................................................................

pub trait Mnode {

    type NodeDef;

    fn create_node() -> Rc<RefCell<Self::NodeDef>>;
    fn mtag(&self) -> usize;
}
//..................................................................................................

/// Short Description
///
/// Longer Description
pub struct VertexDef<const D: usize>
{
    mtag: usize,
    position: Vector<D>,
    out_fin: Option<Fin<D>>,
}

impl<const D: usize> VertexDef<D>
{
    pub fn new() -> Self {
        VertexDef {
            mtag: MUID_NULL,
            position: Vector::zeros(),
            out_fin: None,
        }
    }
}

pub type Vertex<const D: usize> = Rc<RefCell<VertexDef<D>>>;

impl<const D: usize> Mnode for Vertex<D>
{
    type NodeDef = VertexDef<D>;

    fn create_node() -> Rc<RefCell<Self::NodeDef>> {
        Rc::new(RefCell::new(VertexDef::new()))
    }

    fn mtag(&self) -> usize {
        self.as_ref().borrow().mtag
    }

}
//..................................................................................................

/// Short Description
///
/// Longer Description
pub struct FinDef<const D: usize>
{
    mtag: usize,
    twin: Option<Fin<D>>,
    next: Option<Fin<D>>,
    vertex: Option<Vertex<D>>,
    face: Option<Face<D>>,
}

impl<const D: usize> FinDef<D>
{
    pub fn new() -> Self
    {
        FinDef {
            mtag: MUID_NULL,
            twin: None,
            next: None,
            vertex: None,
            face: None,
        }
    }
}

pub type Fin<const D: usize> = Rc<RefCell<FinDef<D>>>;  

impl<const D: usize> Mnode for Fin<D>
{
    type NodeDef = FinDef<D>;

    fn create_node() -> Rc<RefCell<Self::NodeDef>> {
        Rc::new(RefCell::new(FinDef::new()))
    }

    fn mtag(&self) -> usize {
        self.as_ref().borrow().mtag
    }
}
//..................................................................................................

/// Short Description
///
/// Longer Description
pub struct FaceDef<const D: usize>
{
    mtag: usize,
    outer_loops: Vec<Fin<D>>,
    inner_loops: Vec<Fin<D>>,  
}

impl<const D: usize> FaceDef<D>
{
    pub fn new() -> Self
    {
        FaceDef {
            mtag: MUID_NULL,
            outer_loops: Vec::new(),
            inner_loops: Vec::new(),
        }
    }
}

pub type Face<const D: usize> = Rc<RefCell<FaceDef<D>>>;

impl<const D: usize> Mnode for Face<D>
{
    type NodeDef = FaceDef<D>;

    fn create_node() -> Rc<RefCell<Self::NodeDef>> {
        Rc::new(RefCell::new(FaceDef::new()))
    }

    fn mtag(&self) -> usize {
        self.as_ref().borrow().mtag
    }
}
//..................................................................................................

/// Short Description
///
/// Longer Description
pub struct DynMesh<const D: usize>
{
    next_mtag: usize,
    vertices: Vec<Vertex<D>>,
    fins: Vec<Fin<D>>,
    faces: Vec<Face<D>>,
}
//..................................................................................................


impl<const D: usize> DynMesh<D>
{
    pub fn new() -> Self
    {
        DynMesh {
            next_mtag: 0,
            vertices: Vec::new(),
            fins: Vec::new(),
            faces: Vec::new(),
        }
    }

    //...................................
    // Euler Operators
    //...................................

    /// Make-Vertex-Face Euler op
    /// 
    /// The very first operation, creates a vertex and the unbounded face.
    fn make_vert_face(&mut self, point: &Vector<D>) -> (Vertex<D>, Face<D>)
    {
        let v0 = self.add_vertex(point);
        let f0 = self.add_face();
        (v0, f0)
    }

    fn make_edge_vertex(&mut self, v0: &Vertex<D>, point: &Vector<D>) -> (Vertex<D>, Fin<D>)
    {
        let v1 = self.add_vertex(point);
        todo!()
    }

    //...................................
    // Low-level creation of nodes
    //...................................

    fn add_vertex(&mut self, point: &Vector<D>) -> Vertex<D>
    {
        let new_vertex = Vertex::create_node();
        new_vertex.borrow_mut().position = *point;
        new_vertex.borrow_mut().mtag = self.get_next_mtag();
        self.vertices.push(new_vertex.clone());
        new_vertex
    }

    fn add_fin(&mut self) -> Fin<D>
    {
        let new_fin = Fin::create_node();
        new_fin.borrow_mut().mtag = self.get_next_mtag();
        self.fins.push(new_fin.clone());
        new_fin
    }

    fn add_face(&mut self) -> Face<D>
    {
        let new_face = Face::create_node();
        new_face.borrow_mut().mtag = self.get_next_mtag();
        self.faces.push(new_face.clone());
        new_face
    }

    fn get_next_mtag(&mut self) -> usize    
    {
        let next_mtag = self.next_mtag;
        self.next_mtag += 1;
        next_mtag
    }


}



//-------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests
{
  
}