//! The schema lays out the core data structures for representing the topology of the modeller.
//!
//!
//--------------------------------------------------------------------------------------------------


use crate::common::Vec3;
use std::borrow::{Borrow, BorrowMut};
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefCell};


const UID_NULL: usize = usize::MAX;

trait Uid 
{
    fn is_null(&self) -> bool;
    fn null() -> Self;
}

impl Uid for usize {
    fn is_null(&self) -> bool {
        *self == UID_NULL
    }
    fn null() -> Self {
        UID_NULL
    }
}
//..................................................................................................

pub trait Node {

    type NodeDef;

    fn create_node() -> Rc<RefCell<Self::NodeDef>>;
    fn tag(&self) -> usize;
    fn node_id(&self) -> usize;
}
//..................................................................................................


/// The vertex definition.
/// 
/// A vertex is defined by its position in 3D space and the set of fins 
/// that point to it. The vertex is the 0D entity of the schema. There can be two types of vertex: 
/// - A normal vertex, which bounds edges. 
/// - An acorn vertex, which exists on its own and is bounded by a minimal shell.
pub struct VertexDef 
{
    tag: usize,
    node_id: usize,

    /// Point in 3-space of the vertex
    point: Vec3, 
    /// Set of fins which point to this vertex
    fins: Vec<Finw>,
}

impl VertexDef 
{
    pub fn new() -> Self {
        VertexDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            point: Vec3::zeros(),
            fins: Vec::new(),
        }
    }
}

/// Owning pointer to the vertex
pub type Vertex = Rc<RefCell<VertexDef>>;
/// Weak pointer to the vertex
pub type Vertexw = Weak<RefCell<VertexDef>>; 

impl Node for Vertex 
{
    type NodeDef = VertexDef;

    fn create_node() -> Rc<RefCell<Self::NodeDef>> {
        Rc::new(RefCell::new(VertexDef::new()))
    }

    fn tag(&self) -> usize {
        let vertex_ref = self.as_ref().borrow();
        vertex_ref.tag
    }

    fn node_id(&self) -> usize {
        let vertex_ref = self.as_ref().borrow();
        vertex_ref.node_id
    }
}
//..................................................................................................

/// The edge definition, an edge is defined by the set of fins exist on it. Each fin is donated to 
/// the edge 
pub struct EdgeDef
{
    tag: usize, 
    node_id: usize, 

    /// Set of fins attached to the edge in counter-clockwise order
    fins: Vec<Fin>, 

}

impl EdgeDef 
{
    pub fn new() -> Self {
        EdgeDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            fins: Vec::new(),
        }
    }
}

pub type Edge = Rc<RefCell<EdgeDef>>;
pub type Edgew = Weak<RefCell<EdgeDef>>;

impl Node for Edge
{
    type NodeDef = EdgeDef;

    fn create_node() -> Rc<RefCell<Self::NodeDef>> {
        Rc::new(RefCell::new(EdgeDef::new()))
    }

    fn tag(&self) -> usize {
        let edge_ref = self.as_ref().borrow();
        edge_ref.tag
    }

    fn node_id(&self) -> usize {
        let edge_ref = self.as_ref().borrow();
        edge_ref.node_id
    }
}
//..................................................................................................

pub struct FinDef
{

    /// tag in the session 
    tag: usize,
    /// node id in the body
    node_id: usize,

    looop: Option<Loopw>,
    /// forward vertex of find
    forward_vertex: Option<Vertexw>,
    /// Edge to which fin belongs
    edge: Option<Edgew>,
    /// next fin in the loop, this is owning and it keeps the chain alive
    next_in_loop: Option<Finw>,
    /// next fin on edge looking counter-clockwise down edge
    next_around_edge: Option<Finw>,
    /// next fin referencing the vertex of this fin
    next_at_vertex: Option<Finw>,
    /// same direction as edges (+ve), opposite direction as edge (-ve)
    sense: bool,

}

impl FinDef {
    pub fn new() -> Self {
        FinDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            looop: None,
            forward_vertex: None,
            edge: None,
            next_in_loop: None,
            next_around_edge: None,
            next_at_vertex: None,
            sense: true,
        }
    }   
    
}

pub type Fin = Rc<RefCell<FinDef>>;
pub type Finw = Weak<RefCell<FinDef>>;

impl Node for Fin 
{
    type NodeDef = FinDef;

    fn create_node() -> Rc<RefCell<Self::NodeDef>> {
        Rc::new(RefCell::new(FinDef::new()))
    }

    fn tag(&self) -> usize {
        self.as_ref().borrow().tag
    }

    fn node_id(&self) -> usize {
        self.as_ref().borrow().node_id
    }
}
//..................................................................................................


pub struct LoopDef 
{
    tag: usize,
    node_id: usize,

    /// first fin in loop
    fin: Option<Finw>,
    /// Face to which loop belongs
    face: Option<Facew>,

}

impl LoopDef {
    pub fn new() -> Self {
        LoopDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            fin: None,
            face: None,
        }
    }
}

pub type Loop = Rc<RefCell<LoopDef>>;
pub type Loopw = Weak<RefCell<LoopDef>>;


impl Node for Loop
{
    type NodeDef = LoopDef;

    fn create_node() -> Loop {
        Rc::new(RefCell::new(LoopDef::new()))
    }

    fn tag(&self) -> usize {
        let loop_ref = self.as_ref().borrow();
        loop_ref.tag
    }

    fn node_id(&self) -> usize {
        let loop_ref = self.as_ref().borrow();
        loop_ref.node_id
    }
}
//..................................................................................................

pub struct FaceDef
{
    tag: usize,
    node_id: usize,

    /// First loop outer loop, other loops are holes
    loops: Vec<Loop>,
    /// shell of which this is a front face
    front_shell: Option<Shellw>,
    /// shell of which this is a back face
    back_shell: Option<Shellw>, 

}


impl FaceDef {
    pub fn new() -> Self {
        FaceDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            loops: Vec::new(),
            front_shell: None,
            back_shell: None,
        }
    }

    pub fn set_outer_loop(&mut self, looop: Loop)
    {
        self.loops.insert(0, looop);
    }

    pub fn add_inner_loop(&mut self, looop: Loop) {
        self.loops.push(looop);
    }

    pub fn set_front_shell(&mut self, shell: Shell) {
        self.front_shell = Some(Rc::downgrade(&shell));
    }

    pub fn set_back_shell(&mut self, shell: Shell) {
        self.back_shell = Some(Rc::downgrade(&shell));
    }   
}   

pub type Face = Rc<RefCell<FaceDef>>;
pub type Facew = Weak<RefCell<FaceDef>>;

impl Node for Face 
{
    type NodeDef = FaceDef;

    fn create_node() -> Face 
    {
        Rc::new(RefCell::new(FaceDef::new()))
    }
    
    fn tag(&self) -> usize 
    {
        let region_ref = self.as_ref().borrow();
        region_ref.tag
    }
    
    fn node_id(&self) -> usize {
        let region_ref = self.as_ref().borrow();
        region_ref.node_id
    }
}
//..................................................................................................

pub struct ShellDef
{
    tag: usize, 
    node_id: usize,

    /// Acorn vertices
    ac_vertices: Vec<Vertex>,
    /// Wireframe edges
    wf_edges: Vec<Edge>,
    /// Set of front faces, faces with normal into shell region
    /// shells own their front faces
    front_faces: Vec<Face>,
    /// Set of back faces, faces with normal pointer out of shell region
    /// Shells do not own their back faces
    back_faces: Vec<Face>,
    /// Region which this shell bounds
    region: Option<Regionw>,
}

impl ShellDef
{
    pub fn new() -> ShellDef {
        ShellDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            ac_vertices: Vec::new(),
            wf_edges: Vec::new(),
            front_faces: Vec::new(),
            back_faces: Vec::new(),
            region: None, 
        }
    }
}

pub type Shell = Rc<RefCell<ShellDef>>;
pub type Shellw = Weak<RefCell<ShellDef>>;

impl Node for Shell 
{
    type NodeDef = ShellDef;

    fn create_node() ->  Shell
    {
        Rc::new(RefCell::new(ShellDef::new()))
    }
    
    fn tag(&self) -> usize 
    {
        let region_ref = self.as_ref().borrow();
        region_ref.tag
    }
    
    fn node_id(&self) -> usize {
        let region_ref = self.as_ref().borrow();
        region_ref.node_id
    }
}
//..................................................................................................

pub enum RegionMaterial {
    Void, 
    Solid,
}

pub struct RegionDef 
{
    tag: usize, 
    node_id: usize,
    material: RegionMaterial,

    shells: Vec<Shell>,
    body: Option<Bodyw>,

}

impl RegionDef
{
    pub fn new() -> Self {
        RegionDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            material: RegionMaterial::Void,
            shells: Vec::new(),
            body: None,
        }
    }

    pub fn append_shell(&mut self, shell: Shell) {
        self.shells.push(shell);
    }   
}

pub type Region = Rc<RefCell<RegionDef>>;
pub type Regionw = Weak<RefCell<RegionDef>>;


impl Node for Region
{
    type NodeDef = RegionDef;

    fn create_node() -> Region 
    {
        Rc::new(RefCell::new(RegionDef::new()))
    }
    
    fn tag(&self) -> usize 
    {
        let region_ref = self.as_ref().borrow();
        region_ref.tag
    }
    
    fn node_id(&self) -> usize {
        let region_ref = self.as_ref().borrow();
        region_ref.node_id
    }
}
//..................................................................................................

pub struct BodyDef
{
    tag: usize, 
    node_id: usize, 

    /// Set of regions which constitute the body
    regions: Vec<Region>,
    /// Set of non-wireframe edges in body
    edges: Vec<Edge>,
    /// Set of non-acorn vertices 
    vertices: Vec<Vertex>,
}

/// Body is the ref-counted pointer to the BodyDef struct
pub type Body = Rc<RefCell<BodyDef>>;
/// Bodyw is the weak ref-counted pointer to the BodyDef struct
pub type Bodyw = Weak<RefCell<BodyDef>>;

impl BodyDef 
{
    pub fn new() -> Self {
        BodyDef {
            tag: UID_NULL,
            node_id: UID_NULL,
            regions: Vec::new(),
            edges: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub fn num_regions(&self) -> usize {
        self.regions.len()
    }   

    pub fn outer_region(&self) -> Region {
        self.regions.first().unwrap().clone()
    }

    pub fn append_region(&mut self, region: Region) {
        self.regions.push(region);
    }
}

impl Node for Body 
{
    type NodeDef = BodyDef;

    fn create_node() -> Body 
    {
        Rc::new(RefCell::new(BodyDef::new()))
    }
    
    fn tag(&self) -> usize 
    {
        let region_ref = self.as_ref().borrow();
        region_ref.tag
    }
    
    fn node_id(&self) -> usize {
        let region_ref = self.as_ref().borrow();
        region_ref.node_id
    }
}

//..................................................................................................

pub struct Session
{
    bodies: Vec<Body>,
}