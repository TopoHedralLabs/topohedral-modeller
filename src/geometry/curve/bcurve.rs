//! This module contains the definition of the B-spline curve class.
//!
//! B-spline curves are defined by a set of control points and a set of knots and weights.
//! They are particularly useful for representing free-form curves.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{Vec3, Vector};
use crate::geometry::common::{homog, inv_homog, Curve};
use crate::splines::{self as spl, knot_eq};
use crate::utilities::{lower_bound, NDArrayWrapper};
use crate::boxing::ABox;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ constants
pub const BCURVE_DER_MAX: usize = 5;
//}}}
//{{{ struct: BcurveDescriptor
pub struct BcurveDescriptor<const D: usize>
{
    pub p: usize,
    pub knots: Vec<f64>,
    pub cpoints: Vec<Vector<D>>,
    pub cweights: Vec<f64>,
}
//}}}
//{{{ collection: Bcurve
//{{{ struct: Bcurve
#[derive(Clone)]
/// A B-spline curve of dimension `D`.
///
/// B-spline curves are defined by a set of control points, knots, and weights. They are useful for representing free-form curves.
///
/// The `Bcurve` struct contains the following fields:
/// - `p`: The order of the B-spline curve.
/// - `knots`: The knot vector of the B-spline curve.
/// - `cpoints_w`: The control points of the B-spline curve in homogeneous coordinates.
/// - `knot_multiplicites`: The multiplicities of the knots.
/// - `abox`: An optional axis-aligned bounding box for the B-spline curve.
pub struct Bcurve<const D: usize>
where
    [(); D + 1]:,
{
    p: usize,
    knots: Vec<f64>,
    cpoints_w: Vec<Vector<{ D + 1 }>>,
    knot_multiplicites: Vec<(f64, usize)>,
    pub abox: Option<ABox<D>> ,
}
//}}}
//{{{ impl: Bcurve
impl<const D: usize> Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
{

    /// Standard constructor of the Bcurve.
    pub fn new(bcd: &BcurveDescriptor<D>) -> Self
    {
        debug_assert!(bcd.p <= spl::PMAX, "Order too large");
        debug_assert!(bcd.knots.is_sorted(), "knots not sorted");
        debug_assert!(bcd.cweights.iter().all(|&x| x >= 0.0));
        debug_assert!(bcd.cweights.len() == bcd.cpoints.len());
        debug_assert!(bcd.knots.len() == bcd.cpoints.len() + bcd.p + 1);

        let mut points_w = vec![Vector::<{ D + 1 }>::zeros(); bcd.cpoints.len()];

        for i in 0..bcd.cpoints.len()
        {
            points_w[i] = homog(&bcd.cpoints[i], bcd.cweights[i]);
        }

        Self {
            p: bcd.p,
            knots: bcd.knots.clone(),
            cpoints_w: points_w,
            knot_multiplicites: spl::multiplicites(&bcd.knots),
            abox: None,
        }
    }
    //..............................................................................................

    /// Accessor to the order of the curve
    pub fn p(self: &Self) -> usize
    {
        self.p
    }
    //..............................................................................................

    /// Accessor to the knits of the curve
    pub fn knots(&self) -> &[f64]
    {
        &self.knots
    }
    //..............................................................................................

    /// Accessor to the control points in homogeneous coordinates
    pub fn cpoints_w(&self) -> &Vec<Vector<{ D + 1 }>>
    {
        &self.cpoints_w
    }
    //..............................................................................................

    /// Computor of the control points in real coordinates.
    pub fn cpoints(&self) -> Vec<Vector<D>> 
    {
        self.cpoints_w.iter().cloned().map(|v| inv_homog(&v)).collect()
    }
    //..............................................................................................

    /// Returns whether the bcurve is rational and so is a NURBS curve, or is merely a non-rational
    /// Bcurve
    pub fn is_rational(&self) -> bool
    {
        let w = self.cpoints_w[0][D];
        let is_rat = self.cpoints_w.iter().any(|v| v[D] != w);  
        is_rat
    }
    //..............................................................................................

    pub fn multiplicity(&self, u: f64) -> usize {

        let knot_mult_result = self.knot_multiplicites.iter().find(|&x| spl::knot_eq(x.0, u));
        let mult = match knot_mult_result {
            Some(knot_mult) => knot_mult.1,
            None => 0,
        };  
        mult
    }
    //..............................................................................................

    /// Returns the curvature function as a function object which does not borrow the calling object.
    pub fn curvature_fn(&self) -> impl Fn(f64) -> f64 {
        let self_clone = self.clone();
        move |u| self_clone.eval_curvature(u)
    }
}
//}}}
//{{{ impl: Curve for  Bcurve
impl<const D: usize> Curve for Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
{
    //{{{ type: Vector
    type Vector = Vector<D>;
    //}}}
    //{{{ fun: eval
    fn eval(
        &self,
        u: f64,
    ) -> Vector<D>
    {
        debug_assert!(spl::is_member(&self.knots, u));

        let mut pointw_tmp = Vector::<{ D + 1 }>::from_element(0.0);
        let (start, end, _nb) = spl::non_zero_basis(&self.knots, u, self.p);

        let mut basis_funs = [0.0; spl::PMAX];
        spl::eval(&self.knots, u, self.p, &mut basis_funs);

        for i in start..end
        {
            pointw_tmp += basis_funs[i - start] * self.cpoints_w[i];
        }
        let point = inv_homog(&pointw_tmp);
        point
    }
    //..............................................................................................
    //}}}
    //{{{ fun: eval_diff
    fn eval_diff(
        &self,
        u: f64,
        m: usize,
    ) -> Vector<D>
    {
        debug_assert!(spl::is_member(&self.knots, u));

        if m == 0
        {
            self.eval(u)
        }
        else
        {
            let mut diff_loc = [Vector::<D>::zeros(); BCURVE_DER_MAX];
            self.eval_diff_all(u, m, &mut diff_loc);
            diff_loc[m]
        }
    }
    //..............................................................................................
    //}}}
    //{{{ fun: eval_diff_all 
    fn eval_diff_all(
        &self,
        u: f64,
        k: usize,
        ders: &mut [Vector<D>],
    )
    {
        debug_assert!(spl::is_member(&self.knots, u));
        debug_assert!(ders.len() >= k + 1);

        if k == 0
        {
            ders[0] = self.eval(u);
        }
        else
        {
            let dim = k + 1;
            let mut dersw = [Vector::<{ D + 1 }>::zeros(); BCURVE_DER_MAX];
            let (start, _, num_basis) = spl::non_zero_basis(&self.knots, u, self.p);

            let mut basis_ders = [0.0; BCURVE_DER_MAX * BCURVE_DER_MAX];
            spl::eval_diff_all(&self.knots, u, self.p, k, &mut basis_ders);
            let basis_ders_arr = NDArrayWrapper::<'_, f64, 2>::new(&mut basis_ders, &[num_basis, k + 1]);

            for m in 0..k + 1
            // loop over derivatives
            {
                for j in 0..num_basis
                {
                    let nj = basis_ders_arr[&[j, m]];
                    let pwj = self.cpoints_w[start + j];
                    dersw[m] += nj * pwj;
                }
            }

            let mut binom = [0.0; BCURVE_DER_MAX * BCURVE_DER_MAX];
            binom_coeff(k, &mut binom);
            let binom_arr = NDArrayWrapper::<'_, f64, 2>::new(&mut binom, &[dim, dim]);

            let mut ders_loc = [Vector::<D>::zeros(); BCURVE_DER_MAX];
            let w0 = dersw[0][D];
            let mut v = Vector::<D>::zeros();

            for m in 0..k + 1
            {
                v.fill(0.0);
                v.copy_from(&dersw[m].rows(0, D));

                for j in 1..m + 1
                {
                    let wj = dersw[j][D];
                    let bmj = binom_arr[&[m, j]];
                    v -= bmj * wj * ders_loc[m - j];
                }
                ders_loc[m] = v / w0;
                ders[m] = ders_loc[m];
            }
        }
    }
    //..............................................................................................
    //}}}
    //{{{ eval_arclen
    fn eval_arclen(
        &self,
        u1: f64,
        u2: f64,
    ) -> f64
    {
        let out = 0.0;
        out
    }
    //..............................................................................................
    //}}}
    //{{{ is_member
    fn is_member(
            &self,
            u: f64,
        ) -> bool {
        spl::is_member(&self.knots, u)
    }
    //..............................................................................................
    //}}}
    //{{{ fun: dim
    fn dim(&self) -> usize {
        D
    }
    //..............................................................................................
    //}}}
    //{{{ fun: max_der
    fn max_der(&self, u: f64) -> usize {
        if self.is_rational() 
        {
            BCURVE_DER_MAX
        }
        else 
        {
            self.p
        }
    }
    //}}}
    //{{{ fun: param_range
    fn param_range(&self) -> (f64, f64) {
        (self.knots[0], self.knots[self.knots.len() - 1])
    }
    //}}}
}
//}}}
//}}}
//{{{ fun: binom_coeff 
fn binom_coeff(
    n: usize,
    binom: &mut [f64],
)
{
    debug_assert!(binom.len() >= (n + 1) * (n + 1));

    binom.fill(0.0);
    let mut binom_arr = NDArrayWrapper::<'_, f64, 2>::new(binom, &[n + 1, n + 1]);

    for i in 0..n + 1
    {
        binom_arr[&[i, i]] = 1.0;
        binom_arr[&[i, 0]] = 1.0;
    }

    for n2 in 2..n + 1
    {
        for k2 in 1..n2
        {
            binom_arr[&[n2, k2]] = binom_arr[&[n2 - 1, k2 - 1]] + binom_arr[&[n2 - 1, k2]];
        }
    }
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
    use approx::{assert_abs_diff_eq, assert_relative_eq, ulps_eq, AbsDiff};
    use serde::Deserialize;
    use std::fs;

    use crate::geometry::common::Curve;
    use crate::test_utils::test_bcurve::load_bcurve;
    use crate::utilities::NDArrayWrapper;
    use crate::test_utils::{test_bcurve::TestData, convert, de_noise};

    use super::*;


    #[test]
    fn binomcoeff()
    {
        let mut binom = [0.0; 36];
        binom_coeff(5, &mut binom);
        let binom_arr = NDArrayWrapper::<'_, f64, 2>::new(&mut binom, &[6, 6]);
        assert!(ulps_eq!(binom_arr[&[5, 0]], 1.0, max_ulps = 4));
        assert!(ulps_eq!(binom_arr[&[5, 1]], 5.0, max_ulps = 4));
        assert!(ulps_eq!(binom_arr[&[5, 2]], 10.0, max_ulps = 4));
        assert!(ulps_eq!(binom_arr[&[5, 3]], 10.0, max_ulps = 4));
        assert!(ulps_eq!(binom_arr[&[5, 4]], 5.0, max_ulps = 4));
        assert!(ulps_eq!(binom_arr[&[5, 5]], 1.0, max_ulps = 4));
    }

    #[test]
    fn construction()
    {
        let test_data = TestData::new();
        let bcurve = load_bcurve::<3>(3, &test_data);
    }
    //..............................................................................................

    macro_rules! eval {
        ($test_name: ident, $knots: ident, $weights: ident, $cpoints:ident, $points: ident, $dim: expr, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let d = $dim;
                let p = $order;
                let test_data = TestData::new();
                let bcurve = load_bcurve::<$dim>(p, &test_data);

                let points = test_data.$points.values;
                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let point1 = points[idx].clone();
                    let point2 = bcurve.eval(*u);   
                    for i in 0..d
                    {
                        assert_relative_eq!(point1[i], point2[i], max_relative = 1e-12);
                    }
                }
            }
        };
    }
    eval!(
        eval_d2_p1,
        knots_p1,
        weights_p1,
        cpoints_d2_p1,
        points_d2_p1,
        2,
        1
    );
    eval!(
        eval_d2_p2,
        knots_p2,
        weights_p2,
        cpoints_d2_p2,
        points_d2_p2,
        2,
        2
    );
    eval!(
        eval_d2_p3,
        knots_p3,
        weights_p3,
        cpoints_d2_p3,
        points_d2_p3,
        2,
        3
    );
    eval!(
        eval_d2_p4,
        knots_p4,
        weights_p4,
        cpoints_d2_p4,
        points_d2_p4,
        2,
        4
    );
    eval!(
        eval_d3_p1,
        knots_p1,
        weights_p1,
        cpoints_d3_p1,
        points_d3_p1,
        3,
        1
    );
    eval!(
        eval_d3_p2,
        knots_p2,
        weights_p2,
        cpoints_d3_p2,
        points_d3_p2,
        3,
        2
    );
    eval!(
        eval_d3_p3,
        knots_p3,
        weights_p3,
        cpoints_d3_p3,
        points_d3_p3,
        3,
        3
    );
    eval!(
        eval_d3_p4,
        knots_p4,
        weights_p4,
        cpoints_d3_p4,
        points_d3_p4,
        3,
        4
    );
    //..............................................................................................

    macro_rules! eval_diff {
        ($test_name: ident, $knots: ident, $weights: ident, $cpoints:ident, $ders: ident, $dim: expr, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let d = $dim;
                let p = $order;
                let test_data = TestData::new();
                let bcurve = load_bcurve::<$dim>(p, &test_data);
                let ders = test_data.$ders.values;

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let mut ders_all_1 = ders[idx].clone();
                    for k in 0..p + 1
                    {
                        let mut ders1 = &mut ders_all_1[(k * d)..((k + 1) * d)];
                        de_noise(ders1);
                        let mut ders2 = bcurve.eval_diff(*u, k);
                        de_noise(ders2.as_mut_slice());
                        for i in 0..d
                        {
                            assert_relative_eq!(ders1[i], ders2[i], max_relative = 1e-12);
                        }
                    }
                }
            }
        };
    }
    eval_diff!(
        eval_diff_d2_p1,
        knots_p1,
        weights_p1,
        cpoints_d2_p1,
        ders_d2_p1,
        2,
        1
    );
    eval_diff!(
        eval_diff_d2_p2,
        knots_p2,
        weights_p2,
        cpoints_d2_p2,
        ders_d2_p2,
        2,
        2
    );
    eval_diff!(
        eval_diff_d2_p3,
        knots_p3,
        weights_p3,
        cpoints_d2_p3,
        ders_d2_p3,
        2,
        3
    );
    eval_diff!(
        eval_diff_d2_p4,
        knots_p4,
        weights_p4,
        cpoints_d2_p4,
        ders_d2_p4,
        2,
        4
    );
    eval_diff!(
        eval_diff_d3_p1,
        knots_p1,
        weights_p1,
        cpoints_d3_p1,
        ders_d3_p1,
        3,
        1
    );
    eval_diff!(
        eval_diff_d3_p2,
        knots_p2,
        weights_p2,
        cpoints_d3_p2,
        ders_d3_p2,
        3,
        2
    );
    eval_diff!(
        eval_diff_d3_p3,
        knots_p3,
        weights_p3,
        cpoints_d3_p3,
        ders_d3_p3,
        3,
        3
    );
    eval_diff!(
        eval_diff_d3_p4,
        knots_p4,
        weights_p4,
        cpoints_d3_p4,
        ders_d3_p4,
        3,
        4
    );
    //..............................................................................................

    macro_rules! tangent {
        ($test_name: ident, $knots: ident, $weights: ident, $cpoints:ident, $tangents: ident, $dim: expr, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let d = $dim;
                let p = $order;
                let test_data = TestData::new();
                let bcurve = load_bcurve::<$dim>(p, &test_data);
                let tangents = test_data.$tangents.values;

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let mut tangent1 = tangents[idx].clone();
                    de_noise(tangent1.as_mut_slice());
                    let mut tangent2 = bcurve.eval_tangent(*u, false);
                    de_noise(tangent2.as_mut_slice());
                    for i in 0..d
                    {
                        assert_relative_eq!(tangent1[i], tangent2[i], max_relative = 1e-12);
                    }
                }
            }
        };
    }
    tangent!(
        tangent_d2_p1,
        knots_p1,
        weights_p1,
        cpoints_d2_p1,
        tangent_d2_p1,
        2,
        1
    );
    tangent!(
        tangent_d2_p2,
        knots_p2,
        weights_p2,
        cpoints_d2_p2,
        tangent_d2_p2,
        2,
        2
    );
    tangent!(
        tangent_d2_p3,
        knots_p3,
        weights_p3,
        cpoints_d2_p3,
        tangent_d2_p3,
        2,
        3
    );
    tangent!(
        tangent_d2_p4,
        knots_p4,
        weights_p4,
        cpoints_d2_p4,
        tangent_d2_p4,
        2,
        4
    );
    tangent!(
        tangent_d3_p1,
        knots_p1,
        weights_p1,
        cpoints_d3_p1,
        tangent_d3_p1,
        3,
        1
    );
    tangent!(
        tangent_d3_p2,
        knots_p2,
        weights_p2,
        cpoints_d3_p2,
        tangent_d3_p2,
        3,
        2
    );
    tangent!(
        tangent_d3_p3,
        knots_p3,
        weights_p3,
        cpoints_d3_p3,
        tangent_d3_p3,
        3,
        3
    );
    tangent!(
        tangent_d3_p4,
        knots_p4,
        weights_p4,
        cpoints_d3_p4,
        tangent_d3_p4,
        3,
        4
    );
    //..............................................................................................

    macro_rules! normal {
        ($test_name: ident, $knots: ident, $weights: ident, $cpoints:ident, $tangents: ident, $dim: expr, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let p = $order;
                let test_data = TestData::new();
                let bcurve = load_bcurve::<$dim>(p, &test_data);
                let tangents = test_data.$tangents.values;

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let mut tangent = Vector::<$dim>::from_row_slice(&tangents[idx]);
                    tangent = tangent.normalize();
                    let normal = bcurve.eval_normal(*u, true);
                    let dot_product = normal.dot(&tangent);
                    assert_abs_diff_eq!(dot_product, 0.0, epsilon = 1e-9);
                }
            }
        };
    }
    normal!(
        normal_d2_p1,
        knots_p1,
        weights_p1,
        cpoints_d2_p1,
        tangent_d2_p1,
        2,
        1
    );
    normal!(
        normal_d2_p2,
        knots_p2,
        weights_p2,
        cpoints_d2_p2,
        tangent_d2_p2,
        2,
        2
    );
    normal!(
        normal_d2_p3,
        knots_p3,
        weights_p3,
        cpoints_d2_p3,
        tangent_d2_p3,
        2,
        3
    );
    normal!(
        normal_d2_p4,
        knots_p4,
        weights_p4,
        cpoints_d2_p4,
        tangent_d2_p4,
        2,
        4
    );
    normal!(
        normal_d3_p1,
        knots_p1,
        weights_p1,
        cpoints_d3_p1,
        tangent_d3_p1,
        3,
        1
    );
    normal!(
        normal_d3_p2,
        knots_p2,
        weights_p2,
        cpoints_d3_p2,
        tangent_d3_p2,
        3,
        2
    );
    normal!(
        normal_d3_p3,
        knots_p3,
        weights_p3,
        cpoints_d3_p3,
        tangent_d3_p3,
        3,
        3
    );
    normal!(
        normal_d3_p4,
        knots_p4,
        weights_p4,
        cpoints_d3_p4,
        tangent_d3_p4,
        3,
        4
    );
    //..............................................................................................

}
//}}}
