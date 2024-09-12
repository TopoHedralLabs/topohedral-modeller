//! This module implemnents the Euler operators for bodies
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

use crate::common::Vec3;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use super::schema::*;


/// Create a new body with single void region, this is the minimal valid body
pub fn make_region_body() -> Body {

    let by = Body::create_node();
    let re = Region::create_node();
    by.borrow_mut().append_region(re);
    by
}

/// Takes the outer region of body and creates an open shell consisting of one face one ring
/// edge and no vertices
pub fn make_open_shell(rg: &Region) -> Shell {
    let sh = Shell::create_node();
    rg.borrow_mut().append_shell(sh.clone());

    let fa = Face::create_node();
    let lo = Loop::create_node();
    let ed = Edge::create_node();
    let fi = Fin::create_node();

    {
        let mut fa_ref = fa.borrow_mut();
        fa_ref.set_outer_loop(lo);
        fa_ref.set_front_shell(sh.clone());
        fa_ref.set_back_shell(sh.clone());
    }

    {
        // let mut lo_ref = lo.borrow_mut();
        
    }



    sh
}



//-------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests
{

    use super::*;
    
    #[test]
    fn make_region_body_test() {  
        let body = make_region_body();
        assert_eq!(body.borrow().num_regions(), 1);
    }

    #[test]
    fn make_open_shell() {  
        let body = make_region_body();
        assert_eq!(body.borrow().num_regions(), 1);
    }
}