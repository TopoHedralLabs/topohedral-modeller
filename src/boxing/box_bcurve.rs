use crate::common::Vector;
use crate::boxing::common::{ABox, ABoxable};
use crate::geometry::{Curve, CurveMinValOpts};
use crate::geometry::{Bcurve, BCURVE_DER_MAX};

use topohedral_integrate::gauss;
use topohedral_optimisation::d1::{minimize, 
    MinimizeOptions, MinimizeReturns, Method, Bounds};


//{{{ impl<const D: usize> Bcurve<D>
impl<const D: usize> Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
    [(); D * 2]:,
{

    fn compute_box(&mut self)
    {
        //{{{ locals
        let mut mins = [std::f64::MAX; D];
        let mut mins_idx = [0; D];
        let mut maxs = [std::f64::MIN; D];
        let mut max_idx = [0; D];
        let mut min_vals = [0.0; D];
        let mut max_vals = [0.0; D];
        let p = self.p();
        let knots = self.knots();
        //}}}
        //{{{ com: find the min and max values at knots in each dimension
        let n = knots.len();
        let start = p;
        let end = n - p;
        for (i, ui) in knots[start..end].into_iter().enumerate()
        {
            let xi = self.eval(*ui);
            for j in 0..D
            {
                if xi[j] < mins[j]
                {
                    mins[j] = xi[j];
                    mins_idx[j] = i;
                }

                if xi[j] > maxs[j]
                {
                    maxs[j] = xi[j];
                    max_idx[j] = i;
                }
            }
        }
        //}}}
        //{{{ com: create minimisation options struct, initialise it to use the bounded 1D method
        let mut min_scal_opts = CurveMinValOpts{
            tol: 1e-8,
            max_iter: 100,
            bounds: None,
        };
        //}}}
        //{{{ com: For each dimension, bracket the min/max and then perform the minimisation. 
        for j in 0..D 
        {
            //{{{ com: min for dimension j
            let min_idx = mins_idx[j];
            let min_interval = match min_idx
            {
                0 => {
                    (knots[start], knots[start + 2])
                }, 
                _ if min_idx == n-1 => {

                    (knots[n-2], knots[n-1])
                }, 
                _ => {
                    (knots[start + min_idx - 1], knots[start + min_idx + 1])
                }
            };

            let fmin = |u| {
                let xi = self.eval(u);
                xi[j]
            };

            min_scal_opts.bounds = Some(min_interval);
            let min_res = self.min_value_scalar(fmin, &min_scal_opts);
            min_vals[j] = min_res.1;
            //}}}
            //{{{ com: max for dimension j
            let max_idx = max_idx[j];
            let max_interval = match max_idx
            {
                0 => {
                    (knots[start], knots[start + 2])
                }, 
                _ if max_idx == n-1 => {

                    (knots[n-2], knots[n-1])
                }, 
                _ => {
                    (knots[start + max_idx - 1], knots[start + max_idx + 1])
                }
            };
            min_scal_opts.bounds = Some(max_interval);

            let fmax = |u| {
                let xi = self.eval(u);
                -xi[j]
            };

            let max_res = self.min_value_scalar(fmax, &min_scal_opts);
            max_vals[j] = -max_res.1;
            //}}}
        }
        //}}}
        //{{{ com: Finally, assign to the abox field:
        self.abox = Some(ABox::new(min_vals, max_vals));
        //}}}
    }
}
//}}}
//{{{ impl<const D: usize> ABoxable<D> for Bcurve<D>
impl<const D: usize> ABoxable<D> for Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
    [(); D * 2]:,
{

    fn get_box(&mut self) -> &ABox<D>
    {
        if self.abox.is_none() 
        {
            self.compute_box();
        }
        self.abox.as_ref().unwrap()
    }
}
//..................................................................................................
//}}}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
    use super::*;
    use approx::{assert_abs_diff_eq, assert_relative_eq, ulps_eq, AbsDiff};

    use crate::test_utils::test_bcurve::{TestData, load_bcurve};



    #[test]
    fn abox_test()
    {
        let test_data = TestData::new();
        let mut bcurve = load_bcurve::<3>(3, &test_data);
        bcurve.compute_box();


        let mins = [3.5943976280809996e-7,-5.7727909628567495, -1.9466863224019781];
        let maxs = [9.931728335174615, 0.46365419036864053, 1.9533790139511857];
        let abox1: ABox<3> = ABox::<3>::new(mins, maxs);
        let abox2 = bcurve.get_box();

        assert_relative_eq!(abox1.xmin(), abox2.xmin(), epsilon = 1e-9);
        assert_relative_eq!(abox1.xmax(), abox2.xmax(), epsilon = 1e-9);
        assert_relative_eq!(abox1.ymin(), abox2.ymin(), epsilon = 1e-9);
        assert_relative_eq!(abox1.ymax(), abox2.ymax(), epsilon = 1e-9);
        assert_relative_eq!(abox1.zmin(), abox2.zmin(), epsilon = 1e-9);
        assert_relative_eq!(abox1.zmax(), abox2.zmax(), epsilon = 1e-9);

    }
    //..............................................................................................
}
//}}}
