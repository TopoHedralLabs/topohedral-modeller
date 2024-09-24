//! Contains the common definitions used throughout the geometry module, these include traits, 
//! constants and a handful of commmon functions
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Vector, VectorOps, ResConstants};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: Curve
/// This trait models the set of operations on a curve.
pub trait Curve
{
    //{{{ type Vector: VectorOps;
    type Vector: VectorOps;
    //}}}
    //{{{ fun: eval
    /// Evaluates a curve at the parameter value $u$. Therefore it evaluates the parameterisation:
    /// $$
    /// \mathbf{C}(u): \mathbb{R} \rightarrow \mathbb{R}^{D}
    /// $$
    /// where $D \in {2,3}$
    /// # Arguments
    /// * `u` - The curve parameter value
    /// * `point` - slice of length $\geq D$ to which point is written
    fn eval(
        &self,
        u: f64,
    ) -> Self::Vector;
    //}}}
    //{{{ fun: eval_diff
    /// Evalutes the $m$'th derivative of the curve:
    /// $$
    ///     \mathbf{C}^{(m)}(u) = \frac{d^{m}C(u)}{du^{m}}
    /// $$
    /// # Arguments
    /// * `u` - The curve parameter value
    /// * `ders` - A slice of length $\geq D$. The i'th component
    ///                is stored in `ders[i]`
    ///
    fn eval_diff(
        &self,
        u: f64,
        m: usize,
    ) -> Self::Vector;
    //}}}
    //{{{ fun: eval_diff_all
    /// Evalutes the $0$'th to the $m$'th derivative of the curve:
    /// $$
    ///     \left \\{ \mathbf{C}^{(0)}, ..., \mathbf{C}^{(m)}(u) \right \\}
    /// $$
    /// # Arguments
    /// * `u` - The curve parameter value
    /// * `ders` - A slice of length $\geq mD$. The i'th derivative  
    ///                is stored in `ders[i*D..(i+1)*D]`
    ///
    fn eval_diff_all(
        &self,
        u: f64,
        m: usize,
        ders: &mut [Self::Vector],
    );
    //}}}
    //{{{ fun: eval_tangent
    /// Evalutes the tangent to the curve at the parameter value `u`.
    fn eval_tangent(
        &self,
        u: f64,
        normalise: bool,
    ) -> Self::Vector
    {
        debug_assert!(self.is_member(u));

        let mut tan = self.eval_diff(u, 1);
        if normalise
        {
            tan = tan.normalize();
        }
        tan
    }
    //}}}
    //{{{ fun: eval_normal 
    /// Evaluates the normal to the curve at the parameter value `u`.
    fn eval_normal(
        &self,
        u: f64,
        normalise: bool,
    ) -> Self::Vector
    {
        debug_assert!(self.is_member(u));

        match self.dim() 
        {
            2 => {
                let tangent = self.eval_tangent(u, normalise);
                let mut normal = Self::Vector::zeros();  
                normal[0] = -tangent[1];
                normal[1] = tangent[0];
                normal
            }
            3 => {

                if self.max_der(u) < 2 
                {
                    Self::Vector::zeros()
                }
                else 
                {
                    let mut ders = [Self::Vector::zeros(); 3];
                    self.eval_diff_all(u, 2, &mut ders);
                    let ve = ders[1];
                    let acc = ders[2];

                    let b = acc.cross(&ve);
                    let b_norm = b.norm();
                    let v_x_b = ve.cross(&b);
                    let mut normal =  v_x_b * b_norm;

                    if normalise && normal.norm() > f64::RES_LINEAR
                    {
                        normal = normal.normalize();
                    }
                    normal
                }
            }
            _=> panic!("dim mux be 2 or 3"),
        }
    }
    //}}}
    //{{{ fun: eval_binormal
    /// Evaluates the binormal to the cure at the parameter value `u`.
    fn eval_binormal(
        &self,
        u: f64,
        normalise: bool,
    ) -> Self::Vector
    {
        debug_assert!(self.is_member(u));

        match self.dim()
        {
            2 => Self::Vector::zeros(),
            3 => {
                let tan = self.eval_tangent(u, false);
                let normal = self.eval_normal(u, false);
                let mut binorm = tan.cross(&normal);
                if normalise
                {
                    binorm = binorm.normalize();
                }
                binorm
            }, 
            _ => {panic!("D must be 2 or 3")}   
        }
    }
    //}}}
    //{{{ fun: eval_curvature
    /// Evaluates the curvature of the curve at the paramter value `u`    
    fn eval_curvature(
        &self,
        u: f64,
    ) -> f64
    {
        let kappa = if self.max_der(u) >= 2
        {
            let mut ders = [Self::Vector::zeros(); 3];
            self.eval_diff_all(u, 2, &mut ders);
            let ve = ders[1];
            let acc = ders[2];
            let ve_cross_acc_norm = ve.cross(&acc).norm();
            let ve_norm = ve.norm();
            ve_cross_acc_norm / (ve_norm * ve_norm * ve_norm)
        }
        else {
            0.0
        };
        kappa
    }
    //}}}
    //{{{ fun: eval_torsion
    /// Evaluates the torsion of the curve at the parameter value `u`
    fn eval_torsion(
        &self,
        u: f64,
    ) -> f64
    {
        let tau = if self.max_der(u) >= 3
        {
            let mut ders = [Self::Vector::zeros(); 4];
            self.eval_diff_all(u, 3, &mut ders);
            let ve = ders[1];
            let acc = ders[2];
            let jerk = ders[3];
            let ve_cross_acc = ve.cross(&acc);
            let ve_cross_acc_norm = ve_cross_acc.norm();
            ve_cross_acc.dot(&jerk) / (ve_cross_acc_norm * ve_cross_acc_norm)
        }
        else 
        {
            0.0
        };
        tau
    }
    
    //}}}
    //{{{ fun: eval_arclen
    /// Evaluates the arc length of the curve at the parameter value `u`
    fn eval_arclen(
        &self,
        u1: f64,
        u2: f64,
    ) -> f64;
    //}}}
    //{{{ fun: is_member
    /// Determines whether the given parameter value `u` is in the valid range of the curve.
    fn is_member(
        &self,
        u: f64,
    ) -> bool;
    //}}}
    //{{{ fun: dim
    /// Returns the dimension of Euclidian space in which the curve is embedded.
    fn dim(&self) -> usize;
    //}}}
    //{{{ fun: max_der
    /// Returns the maximum allowed order of derivative at the given parameter
    fn max_der(&self, u: f64) -> usize;
    //}}}
    //{{{ fun: min_value_scalar
    /// Finds the minimum value of a scalar function `f` over an optional parameter range.
    ///
    /// This function evaluates the scalar function `f` over an optional parameter range `param_range`
    /// and returns the minimum value and the parameter value at which the minimum occurs.
    ///
    /// # Arguments
    /// * `f` - A closure that takes a `f64` parameter and returns a `f64` value.
    /// * `param_range` - An optional tuple `(f64, f64)` specifying the parameter range over which to
    ///   evaluate the function `f`. If `None`, the function will be evaluated over the entire valid
    ///   parameter range of the object.
    ///
    /// # Returns
    /// A tuple `(f64, f64)` where the second element is the minimum value of `f` and the first element
    /// is the parameter value at which the minimum occurs.
    fn min_value_scalar<F: Fn(f64) -> f64>(&self, f: F, param_range: Option<(f64, f64)>) -> (f64, f64);
    //}}}
    //{{{ fun: min_value_vector
    /// Finds the minimum value of a vector-valued function `f` over an optional parameter range.
    ///
    /// This function evaluates the vector-valued function `f` over an optional parameter range `param_range`
    /// and returns the minimum value of the function.
    ///
    /// # Arguments
    /// * `f` - A closure that takes a `Self::Vector` parameter and returns a `f64` value.
    /// * `param_range` - An optional tuple `(f64, f64)` specifying the parameter range over which to
    ///   evaluate the function `f`. If `None`, the function will be evaluated over the entire valid
    ///   parameter range of the object.
    ///
    /// # Returns
    /// A tuple `(f64, f64)` where the first value is the parameter value at which the mininum occurs 
    /// and the second value is the minimum value of the function `f` over the specified parameter range.
    fn min_value_vector<F: Fn(Self::Vector) -> f64>(&self, f: F, param_range: Option<(f64, f64)>) -> (f64, f64);
    //}}}
    //{{{ fun: integrate_scalar
    /// Integrates a scalar function `f` over an optional parameter range.
    ///
    /// This function evaluates the scalar function `f` over an optional parameter range `param_range`
    /// and returns the integral of the function over the specified range.
    ///
    /// # Arguments
    /// * `f` - A closure that takes a `f64` parameter and returns a `f64` value.
    /// * `param_range` - An optional tuple `(f64, f64)` specifying the parameter range over which to
    ///   evaluate the function `f`. If `None`, the function will be integrated over the entire valid
    ///   parameter range of the object.
    ///
    /// # Returns
    /// The integral of the function `f` over the specified parameter range.
    fn integrate_scalar<F: Fn(f64) -> f64>(&self, f: F, param_range: Option<(f64, f64)>) -> f64;
    //}}}
    //{{{ fun: integrate_vector
    /// Integrates a vector-valued function `f` over an optional parameter range.
    ///
    /// This function evaluates the vector-valued function `f` over an optional parameter range `param_range`
    /// and returns the integral of the function over the specified range.
    ///
    /// # Arguments
    /// * `f` - A closure that takes a `Self::Vector` parameter and returns a `f64` value.
    /// * `param_range` - An optional tuple `(f64, f64)` specifying the parameter range over which to
    ///   evaluate the function `f`. If `None`, the function will be integrated over the entire valid
    ///   parameter range of the object.
    ///
    /// # Returns
    /// The integral of the function `f` over the specified parameter range.
    fn integrate_vector<F: Fn(Self::Vector) -> f64>(&self, f: F, param_range: Option<(f64, f64)>) -> f64;
    //}}}
}
//}}}
//{{{ trait: Surface 
/// This trait models the set of operations on a surface
pub trait Surface
{

    //{{{ type: Vector
    type Vector: VectorOps;
    //}}}
    //{{{ fun: eval
    /// Evaluates a point on the surface.
    ///
    /// Therefore it evaluates the parameterisation:
    /// $$
    /// \mathbf{S}(u, v): \mathbb{R}^{2} \rightarrow  \mathbb{R}^{3}
    /// $$
    fn eval(
        &self,
        u: f64,
        v: f64) -> Self::Vector;
    //}}}
    //{{{ fun: eval_diff_u
    /// Evaluates the ``nu``'th partial derivative of the surface with respect to ``u`` and the
    /// ``nv``'th partial derivative with respect to ``v``.
    ///
    /// Therefore the quantity:
    fn eval_diff_u(
        &self,
        u: f64,
        v: f64, 
        nu: usize,
    ) -> Self::Vector;
    //}}}
    //{{{ fun: eval_diff_v
    /// Evaluates the ``nu``'th partial derivative of the surface with respect to ``u`` and the
    /// ``nv``'th partial derivative with respect to ``v``.
    ///
    /// Therefore the quantity:
    fn eval_diff_v(
        &self,
        u: f64,
        v: f64, 
        nv: usize,
    ) -> Self::Vector;
    //}}}
    //{{{ fun: eval_diff_all
    /// Computes all of the partial derivatives of the surface up to the specified orders.
    ///
    /// Given a surface $\mathbf{s}(u, v)$, the derivative $\mathbf{s}^{(m, l)}(u,v) will be 
    /// stored in `ders[m + nu*l]` where m is the order of the derivative with respect to u and l is
    /// is the order of the derivative with respect to v
    /// # Arguments
    ///
    /// # Returns
    ///
    fn eval_diff_all(
        &self,
        u: f64,
        v: f64,
        nu: usize,
        nv: usize,
        ders: &mut [Self::Vector],
    );
    //}}}
    fn eval_tangent(
        &self,
        u: f64,
        v: f64,
        normalise: bool
    ) -> (Self::Vector, Self::Vector);
    

    fn eval_normal(
        &self,
        u: f64,
        v: f64,
        normalise: bool,
    ) -> Self::Vector;
    

    fn eval_principle_curvatures(
        &self,
        u: f64,
        v: f64,
    ) -> (f64, f64);
    

    fn eval_gauss_curvature(
        &self,
        u: f64,
        v: f64,
    ) -> f64;
    

    fn eval_mean_curvature(
        &self,
        u: f64,
        v: f64,
    ) -> f64;
    

    /// Determines whether the given parameter value `u` is in the valid U-range of the surface.
    fn is_member_u(
        &self,
        u: f64,
    ) -> bool;
    

    /// Determines whether the given parameter value `u` is in the valid V-range of the surface.
    fn is_member_v(
        &self,
        v: f64,
    ) -> bool;
    

    /// Returns the dimension of Euclidian space in which the curve is embedded.
    fn dim(&self) -> usize;
    

    /// Returns the maximum allowed order of derivative at the given parameter
    fn max_der_u(&self, u: f64) -> usize;
    

    /// Returns the maximum allowed order of derivative at the given parameter
    fn max_der_v(&self, v: f64) -> usize;
    
}
//}}}
//{{{ fun: inv_homog
/// Performs the perspective map (inverse of homogeneuos map) from homogeneous coordinates to
/// Euclidean coordinates.
pub fn inv_homog<const N: usize>(point_w: &Vector<{ N + 1 }>) -> Vector<{ N }>
where
    [(); N + 1]:,
{
    let mut point = Vector::<{ N }>::from_element(0.0);
    let w = point_w[N];
    for i in 0..N
    {
        point[i] = point_w[i] / w;
    }
    point
}
//}}}
//{{{ fun: homog
/// Performs the inverse perspective map (homogeneous map) from Euclidean coordinates to
/// Homogeneious coordinates.
pub fn homog<const N: usize>(
    point: &Vector<N>,
    weight: f64,
) -> Vector<{ N + 1 }>
{
    let mut point_w = Vector::<{ N + 1 }>::from_element(0.0);
    for i in 0..N
    {
        point_w[i] = weight * point[i];
    }
    point_w[N] = weight;
    point_w
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}