use crate::boxing::ABox;
use crate::common::Vector;
use crate::geometry::common::{homog, inv_homog};
use crate::splines as spl;

use crate::geometry::common::Surface;

pub const BSURFACE_DER_MAX: usize = spl::PMAX + 1;

pub struct BsurfaceDescriptor<const D: usize>
{
    p: usize,
    q: usize,
    knots_u: Vec<f64>,
    knots_v: Vec<f64>,
    cpoints: Vec<Vector<D>>,
    cweights: Vec<f64>,
}
//..................................................................................................

pub struct Bsurface<const D: usize>
where
    [(); D + 1]:,
{
    p: usize,
    q: usize,
    knots_u: Vec<f64>,
    knots_v: Vec<f64>,
    cpoints_w: Vec<Vector<{ D + 1 }>>,
    r: usize,
    s: usize,
    abox: Option<ABox<D>>,
}
//..................................................................................................

impl<const D: usize> Bsurface<D>
where
    [(); D + 1]:,
    [(); D * BSURFACE_DER_MAX]:,
    [(); D * 3]:,
{
    pub fn new(bsd: &BsurfaceDescriptor<D>) -> Self
    {
        debug_assert!(bsd.p <= spl::PMAX, "Order too large");
        debug_assert!(bsd.knots_u.is_sorted(), "knots not sorted");
        debug_assert!(bsd.knots_v.is_sorted(), "knots not sorted");
        debug_assert!(bsd.cweights.iter().all(|&x| x >= 0.0));
        debug_assert!(bsd.cweights.len() == bsd.cpoints.len());
        debug_assert!(
            (bsd.knots_u.len() - bsd.p - 1) * (bsd.knots_v.len() - bsd.q - 1) == bsd.cpoints.len()
        );

        let mut points_w = vec![Vector::<{ D + 1 }>::zeros(); bsd.cpoints.len()];

        for i in 0..bsd.cpoints.len()
        {
            points_w[i] = homog(&bsd.cpoints[i], bsd.cweights[i]);
        }

        Self {
            p: bsd.p,
            q: bsd.q,
            knots_u: bsd.knots_u.clone(),
            knots_v: bsd.knots_v.clone(),
            cpoints_w: points_w,
            r: bsd.knots_u.len() - bsd.p - 1,
            s: bsd.knots_v.len() - bsd.q - 1,
            abox: None,
        }
    }

    fn pointw(
        &self,
        i: usize,
        j: usize,
    ) -> &Vector<{ D + 1 }>
    {
        &self.cpoints_w[i + j * self.r]
    }
}
//..................................................................................................

impl<const D: usize> Surface for Bsurface<D>
where
    [(); D + 1]:,
    [(); D * BSURFACE_DER_MAX]:,
    [(); D * 3]:,
{
    type Vector = Vector<D>;

    fn eval(
        &self,
        u: f64,
        v: f64
    ) -> Vector<D>
    {
        debug_assert!(spl::is_member(&self.knots_u, u));
        debug_assert!(spl::is_member(&self.knots_v, v));

        let mut pointw_tmp = Vector::<{ D + 1 }>::from_element(0.0);
        let (startu, endu, _) = spl::non_zero_basis(&self.knots_u, u, self.p);
        let (startv, endv, _) = spl::non_zero_basis(&self.knots_v, v, self.q);

        let mut basis_funs_u = [0.0; spl::PMAX];
        spl::eval(&self.knots_u, u, self.p, &mut basis_funs_u);
        let mut basis_funs_v = [0.0; spl::PMAX];
        spl::eval(&self.knots_v, v, self.q, &mut basis_funs_v);

        for j in startv..endv
        {
            let basis_v_j = basis_funs_v[j - startv];

            for i in startu..endu
            {
                let basis_u_i = basis_funs_u[i - startu];
                let pointw_ij = self.pointw(i, j);

                pointw_tmp += (basis_u_i * basis_v_j) * pointw_ij;
            }
        }

        let point = inv_homog(&pointw_tmp);
        point
    }


    fn eval_principle_curvatures(
        &self,
        u: f64,
        v: f64,
    ) -> (f64, f64)
    {
        todo!()
    }

    fn eval_gauss_curvature(
        &self,
        u: f64,
        v: f64,
    ) -> f64
    {
        todo!()
    }

    fn eval_mean_curvature(
        &self,
        u: f64,
        v: f64,
    ) -> f64
    {
        todo!()
    }
    
    
    fn is_member_u(
        &self,
        u: f64,
    ) -> bool {
        todo!()
    }
    
    fn is_member_v(
        &self,
        v: f64,
    ) -> bool {
        todo!()
    }
    
    fn dim(&self) -> usize {
        todo!()
    }
    
    fn max_der_u(&self, u: f64) -> usize {
        todo!()
    }
    
    fn max_der_v(&self, v: f64) -> usize {
        todo!()
    }
    
    fn eval_diff_u(
        &self,
        u: f64,
        v: f64, 
        nu: usize,
    ) -> Self::Vector {
        todo!()
    }
    
    fn eval_diff_v(
        &self,
        u: f64,
        v: f64, 
        nv: usize,
    ) -> Self::Vector {
        todo!()
    }
    
    fn eval_diff_all(
        &self,
        u: f64,
        v: f64,
        nu: usize,
        nv: usize,
        ders: &mut [Self::Vector],
    ) {
        todo!()
    }
    
    fn eval_tangent(
        &self,
        u: f64,
        v: f64,
        normalise: bool
    ) -> (Self::Vector, Self::Vector) {
        todo!()
    }
    
    fn eval_normal(
        &self,
        u: f64,
        v: f64,
        normalise: bool,
    ) -> Self::Vector {
        todo!()
    }
}

//------------------------------------------- tests ----------------------------------------------//

#[cfg(test)]
mod tests
{

    use approx::{assert_abs_diff_eq, assert_relative_eq, ulps_eq};
    use serde::Deserialize;
    use std::fs;

    use crate::geometry::common::Surface;
    use crate::test_utils::{convert, de_noise};
    use crate::utilities::NDArrayWrapper;

    use super::*;

    #[derive(Deserialize)]
    struct ParamData
    {
        description: String,
        values: Vec<Vec<f64>>,
    }

    #[derive(Deserialize)]
    struct KnotData
    {
        description: String,
        values: Vec<f64>,
    }

    #[derive(Deserialize)]
    struct WeightData
    {
        description: String,
        values: Vec<f64>,
    }

    #[derive(Deserialize)]
    struct CpointData
    {
        description: String,
        values: Vec<Vec<f64>>,
    }

    #[derive(Deserialize)]
    struct PointData
    {
        description: String,
        values: Vec<Vec<f64>>,
    }

    #[derive(Deserialize)]
    struct DerData
    {
        description: String,
        values: Vec<Vec<f64>>,
    }

    #[derive(Deserialize)]
    struct TestData
    {
        uv: ParamData,
        knotsu_p1: KnotData,
        knotsu_p2: KnotData,
        knotsu_p3: KnotData,
        knotsu_p4: KnotData,
        knotsu_p5: KnotData,
        knotsv_q2: KnotData,
        knotsv_q3: KnotData,
        knotsv_q4: KnotData,
        knotsv_q5: KnotData,
        knotsv_q6: KnotData,
        weights_p1_q2: WeightData,
        weights_p2_q3: WeightData,
        weights_p3_q4: WeightData,
        weights_p4_q5: WeightData,
        weights_p5_q6: WeightData,
        cpoints_d2_p1_q2: CpointData,
        cpoints_d2_p2_q3: CpointData,
        cpoints_d2_p3_q4: CpointData,
        cpoints_d2_p4_q5: CpointData,
        cpoints_d2_p5_q6: CpointData,
        cpoints_d3_p1_q2: CpointData,
        cpoints_d3_p2_q3: CpointData,
        cpoints_d3_p3_q4: CpointData,
        cpoints_d3_p4_q5: CpointData,
        cpoints_d3_p5_q6: CpointData,
        points_d2_p1_q2: PointData,
        points_d2_p2_q3: PointData,
        points_d2_p3_q4: PointData,
        points_d2_p4_q5: PointData,
        points_d2_p5_q6: PointData,
        points_d3_p1_q2: PointData,
        points_d3_p2_q3: PointData,
        points_d3_p3_q4: PointData,
        points_d3_p4_q5: PointData,
        points_d3_p5_q6: PointData,
        ders_d2_p1_q2: DerData,
        ders_d2_p2_q3: DerData,
        ders_d2_p3_q4: DerData,
        ders_d2_p4_q5: DerData,
        ders_d2_p5_q6: DerData,
        ders_d3_p1_q2: DerData,
        ders_d3_p2_q3: DerData,
        ders_d3_p3_q4: DerData,
        ders_d3_p4_q5: DerData,
        ders_d3_p5_q6: DerData,
    }

    impl TestData
    {
        pub fn new() -> Self
        {
            let json_file =
                fs::read_to_string("assets/geo/bsurface-tests.json").expect("Unable to read file");
            serde_json::from_str(&json_file).expect("Could not deserialize")
        }
    }

    #[test]
    fn construction_2d()
    {
        let test_data = TestData::new();
        let p = 1;
        let q = 2;
        let knotsu = test_data.knotsu_p1.values;
        let knotsv = test_data.knotsv_q2.values;
        let cpoints: Vec<Vector<2>> = convert(&test_data.cpoints_d2_p1_q2.values);
        let cweights = test_data.weights_p1_q2.values;

        let descriptor = BsurfaceDescriptor {
            p: p,
            q: q,
            knots_u: knotsu,
            knots_v: knotsv,
            cpoints: cpoints,
            cweights: cweights,
        };
        let bsurf = Bsurface::<2>::new(&descriptor);
    }

    #[test]
    fn construction_3d()
    {
        let test_data = TestData::new();
        let p = 1;
        let q = 2;
        let knotsu = test_data.knotsu_p1.values;
        let knotsv = test_data.knotsv_q2.values;
        let cpoints: Vec<Vector<3>> = convert(&test_data.cpoints_d3_p1_q2.values);
        let cweights = test_data.weights_p1_q2.values;

        let descriptor = BsurfaceDescriptor {
            p: p,
            q: q,
            knots_u: knotsu,
            knots_v: knotsv,
            cpoints: cpoints,
            cweights: cweights,
        };
        let bsurf = Bsurface::<3>::new(&descriptor);
    }

    macro_rules! eval {
        ($test_name: ident, 
         $knotsu: ident, 
         $knotsv: ident, 
         $weights: ident, 
         $cpoints:ident, 
         $points: ident, 
         $dim: expr, 
         $orderu:expr,
         $orderv:expr) => {

            #[test] 
            fn $test_name() {

                let test_data = TestData::new();

                let d = $dim;
                let p = $orderu;
                let q = $orderv;
                let knotsu = test_data.$knotsu.values;
                let knotsv = test_data.$knotsv.values;
                let weights = test_data.$weights.values;
                let cpoints: Vec<Vector<$dim>> = convert(&test_data.$cpoints.values);

                let descriptor = BsurfaceDescriptor {
                    p: p,
                    q: q,
                    knots_u: knotsu,
                    knots_v: knotsv,
                    cpoints: cpoints,
                    cweights: weights,
                };
                let bsurf = Bsurface::<$dim>::new(&descriptor);

                let points = test_data.$points.values;

                for (idx, uv) in test_data.uv.values.iter().enumerate()
                {
                    let u = uv[0];
                    let v = uv[1];
                    let point1 = points[idx].clone();

                    let mut point2 = Vector::<$dim>::zeros();
                    let point2 = bsurf.eval(u, v);

                    for i in 0..d
                    {
                        assert_relative_eq!(point1[i], point2[i], epsilon = 1e-10);
                    }
                }
            }
         };

    }
    eval!(
        eval_d2_p1_q2, 
        knotsu_p1, 
        knotsv_q2,
        weights_p1_q2,
        cpoints_d2_p1_q2,
        points_d2_p1_q2,
        2, 
        1, 
        2
     );
    eval!(
        eval_d2_p2_q3, 
        knotsu_p2, 
        knotsv_q3,
        weights_p2_q3,
        cpoints_d2_p2_q3,
        points_d2_p2_q3,
        2, 
        2, 
        3
     );
    eval!(
        eval_d2_p3_q4, 
        knotsu_p3, 
        knotsv_q4,
        weights_p3_q4,
        cpoints_d2_p3_q4,
        points_d2_p3_q4,
        2, 
        3, 
        4
     );
    eval!(
        eval_d2_p4_q5, 
        knotsu_p4, 
        knotsv_q5,
        weights_p4_q5,
        cpoints_d2_p4_q5,
        points_d2_p4_q5,
        2, 
        4, 
        5
     );
    eval!(
        eval_d2_p5_q6, 
        knotsu_p5, 
        knotsv_q6,
        weights_p5_q6,
        cpoints_d2_p5_q6,
        points_d2_p5_q6,
        2, 
        5, 
        6
     );
    eval!(
        eval_d3_p1_q2, 
        knotsu_p1, 
        knotsv_q2,
        weights_p1_q2,
        cpoints_d3_p1_q2,
        points_d3_p1_q2,
        3, 
        1, 
        2
     );
    eval!(
        eval_d3_p2_q3, 
        knotsu_p2, 
        knotsv_q3,
        weights_p2_q3,
        cpoints_d3_p2_q3,
        points_d3_p2_q3,
        3, 
        2, 
        3
     );
    eval!(
        eval_d3_p3_q4, 
        knotsu_p3, 
        knotsv_q4,
        weights_p3_q4,
        cpoints_d3_p3_q4,
        points_d3_p3_q4,
        3, 
        3, 
        4
     );
    eval!(
        eval_d3_p4_q5, 
        knotsu_p4, 
        knotsv_q5,
        weights_p4_q5,
        cpoints_d3_p4_q5,
        points_d3_p4_q5,
        3, 
        4, 
        5
     );
    eval!(
        eval_d3_p5_q6, 
        knotsu_p5, 
        knotsv_q6,
        weights_p5_q6,
        cpoints_d3_p5_q6,
        points_d3_p5_q6,
        3, 
        5, 
        6
     );
    //.............................................................................................

    macro_rules! eval_diff {
        ($test_name: ident, 
         $knotsu: ident, 
         $knotsv: ident, 
         $weights: ident, 
         $cpoints:ident, 
         $ders: ident, 
         $dim: expr, 
         $orderu:expr,
         $orderv:expr) => {

            #[test] 
            fn $test_name() {

                let test_data = TestData::new();

                let max_deriv = 4;
                let d = $dim;
                let p = $orderu;
                let q = $orderv;
                let knotsu = test_data.$knotsu.values;
                let knotsv = test_data.$knotsv.values;
                let weights = test_data.$weights.values;
                let cpoints: Vec<Vector<$dim>> = convert(&test_data.$cpoints.values);

                let descriptor = BsurfaceDescriptor {
                    p: p,
                    q: q,
                    knots_u: knotsu,
                    knots_v: knotsv,
                    cpoints: cpoints,
                    cweights: weights,
                };
                let bsurf = Bsurface::<$dim>::new(&descriptor);

                let ders = test_data.$ders.values;

                for (idx, uv) in test_data.uv.values.iter().enumerate()
                {
                    let u = uv[0];
                    let v = uv[1];

                    let start = (max_deriv * max_deriv) * idx;
                    let end =  (max_deriv * max_deriv) * (idx+1);
                    let ders_all_1 = ders[start..end].to_vec();

                    let mut point2 = Vector::<$dim>::zeros();
                    bsurf.eval(u, v, point2.as_mut_slice());

                    // for i in 0..d
                    // {
                    //     assert_relative_eq!(point1[i], point2[i], epsilon = 1e-10);
                    // }
                }
            }
         };
    }

    // eval_diff!(
    //     eval_diff_d2_p1_q2, 
    //     knotsu_p1, 
    //     knotsv_q2,
    //     weights_p1_q2,
    //     cpoints_d2_p1_q2,
    //     points_d2_p1_q2,
    //     2, 
    //     1, 
    //     2
    //  );
    // eval_diff!(
    //     eval_diff_d2_p2_q3, 
    //     knotsu_p2, 
    //     knotsv_q3,
    //     weights_p2_q3,
    //     cpoints_d2_p2_q3,
    //     points_d2_p2_q3,
    //     2, 
    //     2, 
    //     3
    //  );
    // eval_diff!(
    //     eval_diff_d2_p3_q4, 
    //     knotsu_p3, 
    //     knotsv_q4,
    //     weights_p3_q4,
    //     cpoints_d2_p3_q4,
    //     points_d2_p3_q4,
    //     2, 
    //     3, 
    //     4
    //  );
    // eval_diff!(
    //     eval_diff_d2_p4_q5, 
    //     knotsu_p4, 
    //     knotsv_q5,
    //     weights_p4_q5,
    //     cpoints_d2_p4_q5,
    //     points_d2_p4_q5,
    //     2, 
    //     4, 
    //     5
    //  );
    // eval_diff!(
    //     eval_diff_d2_p5_q6, 
    //     knotsu_p5, 
    //     knotsv_q6,
    //     weights_p5_q6,
    //     cpoints_d2_p5_q6,
    //     points_d2_p5_q6,
    //     2, 
    //     5, 
    //     6
    //  );
    // eval_diff!(
    //     eval_diff_d3_p1_q2, 
    //     knotsu_p1, 
    //     knotsv_q2,
    //     weights_p1_q2,
    //     cpoints_d3_p1_q2,
    //     points_d3_p1_q2,
    //     3, 
    //     1, 
    //     2
    //  );
    // eval_diff!(
    //     eval_diff_d3_p2_q3, 
    //     knotsu_p2, 
    //     knotsv_q3,
    //     weights_p2_q3,
    //     cpoints_d3_p2_q3,
    //     points_d3_p2_q3,
    //     3, 
    //     2, 
    //     3
    //  );
    // eval_diff!(
    //     eval_diff_d3_p3_q4, 
    //     knotsu_p3, 
    //     knotsv_q4,
    //     weights_p3_q4,
    //     cpoints_d3_p3_q4,
    //     points_d3_p3_q4,
    //     3, 
    //     3, 
    //     4
    //  );
    // eval_diff!(
    //     eval_diff_d3_p4_q5, 
    //     knotsu_p4, 
    //     knotsv_q5,
    //     weights_p4_q5,
    //     cpoints_d3_p4_q5,
    //     points_d3_p4_q5,
    //     3, 
    //     4, 
    //     5
    //  );
    // eval_diff!(
    //     eval_diff_d3_p5_q6, 
    //     knotsu_p5, 
    //     knotsv_q6,
    //     weights_p5_q6,
    //     cpoints_d3_p5_q6,
    //     points_d3_p5_q6,
    //     3, 
    //     5, 
    //     6
    //  );
    //.............................................................................................
}
