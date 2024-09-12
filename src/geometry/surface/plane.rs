use crate::{common::{
    vec_colinear, vec_orthogonal, vec_unitary, Descriptor, DescriptorError, ResConstants, Vec3,
}, utilities};

use crate::geometry::{common::Surface, Curve};

pub struct PlaneDescriptor
{
    pub origin: Vec3,
    pub x: Vec3,
    pub y: Vec3,
}

impl Descriptor for PlaneDescriptor
{
    fn is_valid(&self) -> Result<(), DescriptorError>
    {
        if !vec_unitary(&self.x, -1.0)
        {
            return Err(DescriptorError::InvalidInput(
                "x vector not unitary".to_string(),
            ));
        }
        if !vec_unitary(&self.y, -1.0)
        {
            return Err(DescriptorError::InvalidInput(
                "x vector not unitary".to_string(),
            ));
        }
        if !vec_orthogonal(&self.x, &self.y, -1.0)
        {
            return Err(DescriptorError::InvalidInput(
                "x and y vectors are not orthogonal".to_string(),
            ));
        }
        Ok(())
    }
}

pub struct Plane
{
    origin: Vec3,
    x: Vec3,
    y: Vec3,
    z: Vec3,
}

impl Plane
{
    pub fn new(pd: &PlaneDescriptor) -> Self
    {
        debug_assert!(pd.is_valid().is_ok(), "Invalid plane descriptor");

        let z = pd.x.cross(&pd.y);
        Plane {
            origin: pd.origin,
            x: pd.x,
            y: pd.y,
            z: z,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn x(&self) -> Vec3 {
        self.x
    }

    pub fn y(&self) -> Vec3 {
        self.y
    }
}

impl Surface for Plane
{
    type Vector = Vec3;

    fn eval(
        &self,
        u: f64,
        v: f64,
    ) -> Self::Vector
    {
        let out = self.origin + u*self.x + v * self.y;
        out
    }

    fn eval_diff_u(
        &self,
        u: f64,
        v: f64,
        nu: usize,
    ) -> Self::Vector
    {
        match nu {
            0 => self.eval(u, v),
            1 => self.x,
            _ => Vec3::zeros(),
        }
    }

    fn eval_diff_v(
        &self,
        u: f64,
        v: f64,
        nv: usize,
    ) -> Self::Vector
    {
        match nv {
            0 => self.eval(u, v),
            1 => self.y,
            _ => Vec3::zeros(),
        }
    }

    fn eval_diff_all(
        &self,
        u: f64,
        v: f64,
        nu: usize,
        nv: usize,
        ders: &mut [Self::Vector],
    )
    {
        debug_assert!(ders.len() >= nu * nv, "Output array is not large enough");

        let hlp = utilities::IndexHelper::<2>::new(&[nu, nv]);
        for j in 0..nv {
            for i in 0..nu 
            {
                let idx = hlp.lin_index(&[i, j]);
                ders[idx] = match (i, j) {
                    (0, 0) => self.eval(u, v),
                    (1, 0) => self.x,
                    (0, 1) => self.y,
                    _ => Vec3::zeros(),
                };
            }
        }
    }

    fn eval_tangent(
        &self,
        u: f64,
        v: f64,
        normalise: bool,
    ) -> (Self::Vector, Self::Vector)
    {
        (self.x, self.y)
    }

    fn eval_normal(
        &self,
        u: f64,
        v: f64,
        normalise: bool,
    ) -> Self::Vector
    {
        self.z
    }

    fn eval_principle_curvatures(
        &self,
        u: f64,
        v: f64,
    ) -> (f64, f64)
    {
        (0.0, 0.0)
    }

    fn eval_gauss_curvature(
        &self,
        u: f64,
        v: f64,
    ) -> f64
    {
        0.0
    }

    fn eval_mean_curvature(
        &self,
        u: f64,
        v: f64,
    ) -> f64
    {
        0.0
    }

    fn is_member_u(
        &self,
        u: f64,
    ) -> bool
    {
        true
    }

    fn is_member_v(
        &self,
        v: f64,
    ) -> bool
    {
        true
    }

    fn dim(&self) -> usize
    {
        3
    }

    fn max_der_u(
        &self,
        u: f64,
    ) -> usize
    {
        1
    }

    fn max_der_v(
        &self,
        v: f64,
    ) -> usize
    {
        1
    }
}

//-------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn invalid_descriptor_test1()
    {
        let pd = PlaneDescriptor {
            origin: Vec3::new(1.0, 2.0, 3.0),
            x: Vec3::new(1.0, 0.0, 0.0),
            y: Vec3::new(0.0, 2.0, 0.0),
        };
        assert!(pd.is_valid().is_err());
    }

    #[test]
    fn invalid_descriptor_test2()
    {
        let pd = PlaneDescriptor {
            origin: Vec3::new(1.0, 2.0, 3.0),
            x: Vec3::new(1.0, 0.0, 0.0),
            y: Vec3::new(1.0, 1.0, 1.0) / 2.0f64.sqrt(),
        };
        assert!(pd.is_valid().is_err());
    }

    #[test]
    fn plane_new_test()
    {
        let pd = PlaneDescriptor {
            origin: Vec3::new(1.0, 2.0, 3.0),
            x: Vec3::new(1.0, 0.0, 0.0),
            y: Vec3::new(0.0, 1.0, 0.0),
        };
        let plane = Plane::new(&pd);
    }
}
