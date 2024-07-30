//! This is the SPLline module
//! In it we compute:
//! - Non-Unform Rational Bsplines basis functions
//!
//!

use crate::utilities::NDArrayWrapper;
use approx::ulps_eq;

/// This is the maximum allowable order of a bspline basis. It is an arbitrary number.
pub const PMAX: usize = 8;
/// This is the tolerance with which two knots are considered equal
pub const KNOT_ULPS: u32 = 32;

/// Tolerant less-thant for knots
fn knot_lt(
    u1: f64,
    u2: f64,
) -> bool
{
    u1 < u2 || ulps_eq!(u1, u2, max_ulps = KNOT_ULPS)
}
//..............................................................................................

/// Tolerant greater-thant for knots
fn knot_gt(
    u1: f64,
    u2: f64,
) -> bool
{
    u1 > u2 || ulps_eq!(u1, u2, max_ulps = KNOT_ULPS)
}
//..............................................................................................

/// Checks if two floating-point values are considered equal within a tolerance.
pub fn knot_eq(
    u1: f64,
    u2: f64,
) -> bool
{
    ulps_eq!(u1, u2, max_ulps = KNOT_ULPS)
}
//..............................................................................................

/// Finds the upper bound index for a given value in a sorted array of floating-point numbers.
///
/// This function uses a binary search algorithm to find the insertion point for `value` in `arr`.
/// It takes into account the custom comparison function `knot_lt` for floating-point values.
///
/// # Parameters
///
/// - `arr`: A slice of f64 values, assumed to be sorted in ascending order.
/// - `value`: The f64 value to find the upper bound for.
///
/// # Returns
///
/// The index where `value` should be inserted to maintain sorted order.
fn upper_bound(
    arr: &[f64],
    value: f64,
) -> usize
{
    let comp = |val: &f64| {
        if knot_lt(*val, value)
        {
            std::cmp::Ordering::Less
        }
        else
        {
            std::cmp::Ordering::Greater
        }
    };
    arr.binary_search_by(comp).unwrap_or_else(|index| index)
}
//..............................................................................................

/// Checks if a given parameter value is within the range of the knot vector.
///
/// This function determines whether a parameter value `u` is within the range defined by the
/// knot vector, using tolerant comparisons for floating-point values.
///
/// # Parameters
///
/// - `knots`: A slice of f64 values representing the knot vector.
/// - `u`: The parameter value to check for membership.
///
/// # Returns
///
/// Returns `true` if `u` is within the range of the knot vector, `false` otherwise.
pub fn is_member(
    knots: &[f64],
    u: f64,
) -> bool
{
    let umin = knots.first().unwrap();
    let umax = knots.last().unwrap();
    knot_gt(u, *umin) && knot_lt(u, *umax)
}
//..............................................................................................

/// Finds the index of the knot vector that contains the given parameter value `u`.
///
/// This function determines the index of the knot vector that contains the given parameter value `u`,
/// assuming the knot vector is sorted in ascending order. It uses a binary search algorithm to
/// efficiently locate the appropriate index.
///
/// # Parameters
///
/// - `knots`: A slice of `f64` values representing the knot vector.
/// - `u`: The parameter value to find the containing knot vector index for.
/// - `p`: The degree of the spline.
///
/// # Returns
///
/// The index of the knot vector that contains the given parameter value `u`.
pub fn find_span(
    knots: &[f64],
    u: f64,
    p: usize,
) -> usize
{
    let n = knots.len() - p - 1;
    let mut span: usize;

    if knot_eq(u, knots[n])
    {
        span = n - 1;
    }
    else
    {
        let low = p;
        let idx = upper_bound(&knots[low..], u) + low;
        span = idx - 1;
    }
    span
}
//..............................................................................................

/// Finds the indices of the non-zero basis functions for the given parameter value `u`.
///
/// This function determines the indices of the non-zero basis functions for the given parameter value `u`,
/// assuming the knot vector is sorted in ascending order. It uses the `find_span` function to efficiently
/// locate the appropriate index range.
///
/// # Parameters
///
/// - `knots`: A slice of `f64` values representing the knot vector.
/// - `u`: The parameter value to find the non-zero basis function indices for.
/// - `p`: The degree of the spline.
///
/// # Returns
///
/// A tuple containing the start index, end index, and the number of non-zero basis functions.
pub fn non_zero_basis(
    knots: &[f64],
    u: f64,
    p: usize,
) -> (usize, usize, usize)
{
    debug_assert!(is_member(knots, u));

    let mi = knots.len() as i32;
    let span_i = find_span(knots, u, p) as i32;
    let pi = p as i32;

    let start = std::cmp::max(0, span_i - pi) as usize;
    let end = (std::cmp::min(span_i, mi - 2 - pi) + 1) as usize;
    let num_basis = end - start;
    (start, end, num_basis)
}
//..............................................................................................

/// Evaluates the B-spline basis functions for the given parameter value `u`.
///
/// This function computes the values of the B-spline basis functions for the given parameter value `u`,
/// assuming the knot vector is sorted in ascending order. It uses the `find_span` function to efficiently
/// locate the appropriate index range.
///
/// # Parameters
///
/// - `knots`: A slice of `f64` values representing the knot vector.
/// - `u`: The parameter value to evaluate the basis functions for.
/// - `p`: The degree of the spline.
/// - `shape_funs`: A mutable slice of `f64` values to store the computed basis function values.
///
/// # Panics
///
/// - If `shape_funs.len() < p + 1`, indicating the buffer is too small to hold the results.
/// - If `u` is not a member of the parameter range defined by the knot vector.
pub fn eval(
    knots: &[f64],
    u: f64,
    p: usize,
    shape_funs: &mut [f64],
)
{
    debug_assert!(
        shape_funs.len() >= p + 1,
        "Buffer too small to hold results"
    );
    debug_assert!(is_member(knots, u), "u is outside of ");

    shape_funs.fill(0.0);
    shape_funs[0] = 1.0;

    let mut left = [0.0; PMAX];
    let mut right = [0.0; PMAX];

    let i = find_span(knots, u, p);

    for j in 1..p + 1
    {
        left[j - 1] = u - knots[i - p + j];
        right[j - 1] = knots[i + j] - u;
    }

    for j in 1..p + 1
    {
        let mut saved = 0.0;
        for r in 0..j
        {
            let ri = right[r];
            let le = left[p - j + r];
            let temp = shape_funs[r] / (ri + le);
            shape_funs[r] = saved + (ri * temp);
            saved = le * temp;
        }
        shape_funs[j] = saved;
    }
}
//..............................................................................................

/// Evaluates the derivatives of the B-spline basis functions up to the `k`-th order.
///
/// # Parameters
///
/// - `knots`: The knot vector.
/// - `u`: The parameter value at which to evaluate the derivatives.
/// - `p`: The degree of the spline.
/// - `k`: The order of the derivative to compute.
/// - `shape_ders`: A mutable slice to store the computed basis function derivatives.
///
/// # Panics
///
/// - If `u` is not a member of the parameter range defined by the knot vector.
/// - If `p` exceeds the maximum allowable order (`PMAX`).
/// - If the length of `shape_ders` is less than `(p + 1)`, indicating the buffer is too small to hold the results.
pub fn eval_diff(
    knots: &[f64],
    u: f64,
    p: usize,
    k: usize,
    shape_ders: &mut [f64],
)
{
    debug_assert!(is_member(knots, u), "u is not member of parameter range");
    debug_assert!(p <= PMAX, "order exceeds max allowable");
    // debug_assert!(k <= p, "order of derivative exceeds order");
    debug_assert!(shape_ders.len() >= (p + 1), "Derivative buffer too small");

    let i = find_span(knots, u, p);
    let p_k = p - k;
    shape_ders.fill(0.0);
    if k <= p
    {
        /* doc: compute shape functions
        This code-block initialises the first p_k+1 elements of
        shape_ders to the shape functions:

            N_{i-p_k, p_k}^{(0)} ... N_{i-p_k, p_k}^{(0)}

        This is the first phase of the cox-deboor recursion.
        */
        shape_ders[0] = 1.0;
        let mut left = [0.0; PMAX];
        let mut right = [0.0; PMAX];
        for j in 1..p_k + 1
        {
            left[j - 1] = u - knots[i - p_k + j];
            right[j - 1] = knots[i + j] - u;
        }
        for j in 1..p_k + 1
        {
            let mut saved = 0.0;
            for r in 0..j
            {
                let ri = right[r];
                let le = left[p_k - j + r];
                let temp = shape_ders[r] / (ri + le);
                shape_ders[r] = saved + (ri * temp);
                saved = le * temp;
            }
            shape_ders[j] = saved;
        }
    }

    for k2 in 1..k + 1
    // loop over derivative
    {
        let p_k2 = p_k + k2;
        let p_k2_f = p_k2 as f64;
        let mut saved = 0.0;
        for r in 0..p_k2
        {
            let denom = knots[i + 1 + r] - knots[i + 1 + r - p_k2];
            let temp = (p_k2_f / denom) * shape_ders[r];
            shape_ders[r] = saved - temp;
            saved = temp;
        }
        shape_ders[p_k2] = saved;
    }
}
//..............................................................................................

pub fn eval_diff_all(
    knots: &[f64],
    u: f64,
    p: usize,
    k_in: usize,
    shape_ders: &mut [f64],
)
{
    debug_assert!(is_member(knots, u), "u is not member of parameter range");
    debug_assert!(p <= PMAX, "order exceeds max allowable");
    // debug_assert!(k <= p, "order of derivative exceeds order");
    debug_assert!(
        shape_ders.len() >= (k_in + 1) * (p + 1),
        "Derivative buffer too small"
    );

    let k = p.min(k_in);
    shape_ders.fill(0.0);
    let mut shape_ders_arr = NDArrayWrapper::<f64, 2>::new(shape_ders, &[p + 1, k + 1]);

    let mut shape_funs = [0.0; PMAX + 1];
    eval(knots, u, p, &mut shape_funs);
    for j in 0..p + 1
    {
        shape_ders_arr[&[j, 0]] = shape_funs[j];
    }

    for k2 in 1..p + 1
    {
        let mut shape_ders_loc = [0.0; PMAX + 1];
        eval_diff(knots, u, p, k2, &mut shape_ders_loc);
        for j in 0..p + 1
        {
            shape_ders_arr[&[j, k2]] = shape_ders_loc[j];
        }
    }
}

pub fn multiplicites(knots: &[f64]) -> Vec<(f64, usize)>
{
    let mut out = Vec::new();
    out.reserve(knots.len());

    let mut cur_knot = knots[0];
    let mut cur_mult = 0;

    for i in 1..knots.len()
    {
        cur_mult += 1;
        if (!knot_eq(knots[i], cur_knot))
        {
            out.push((cur_knot, cur_mult));
            cur_knot = knots[i];
            cur_mult = 0;
        }
    }

    out.push((cur_knot, cur_mult+1));
    out
}
//..............................................................................................

// ------------------------------------------- Tests -------------------------------------------- //
#[cfg(test)]
mod tests
{

    use super::*;

    use approx::assert_relative_eq;
    use serde::Deserialize;
    use std::fs;

    const ZERO_THRESHOLD: f64 = 1e-14;

    #[derive(Deserialize)]
    struct ParamData
    {
        description: String,
        values: Vec<f64>,
    }

    #[derive(Deserialize)]
    struct KnotData
    {
        description: String,
        values: Vec<f64>,
    }

    #[derive(Deserialize)]
    struct SpanData
    {
        description: String,
        values: Vec<usize>,
    }

    #[derive(Deserialize)]
    struct BasisData
    {
        description: String,
        values: Vec<Vec<f64>>,
    }

    #[derive(Deserialize)]
    struct TestData
    {
        u: ParamData,
        knots_p0: KnotData,
        knots_p1: KnotData,
        knots_p2: KnotData,
        knots_p3: KnotData,
        knots_p4: KnotData,
        span_p0: SpanData,
        span_p1: SpanData,
        span_p2: SpanData,
        span_p3: SpanData,
        span_p4: SpanData,
        basis_p0: BasisData,
        basis_p1: BasisData,
        basis_p2: BasisData,
        basis_p3: BasisData,
        basis_p4: BasisData,
        ders_p0: BasisData,
        ders_p1: BasisData,
        ders_p2: BasisData,
        ders_p3: BasisData,
        ders_p4: BasisData,
    }

    impl TestData
    {
        pub fn new() -> Self
        {
            let json_file =
                fs::read_to_string("assets/spl/bsplinebasis.json").expect("Unable to read file");
            serde_json::from_str(&json_file).expect("Could not deserialize")
        }
    }

    macro_rules! find_span {
        ($test_name:ident, $knots:ident, $span:ident, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let test_data = TestData::new();
                let knots = test_data.$knots.values.clone();

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let span1 = test_data.$span.values[idx];
                    let span2 = find_span(&knots, *u, $order);
                    assert_eq!(span1, span2);
                }
            }
        };
    }
    find_span!(find_span0, knots_p0, span_p0, 0);
    find_span!(find_span1, knots_p1, span_p1, 1);
    find_span!(find_span2, knots_p2, span_p2, 2);
    find_span!(find_span3, knots_p3, span_p3, 3);
    find_span!(find_span4, knots_p4, span_p4, 4);
    //..............................................................................................

    macro_rules! eval {
        ($test_name:ident, $knots:ident, $basis:ident, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let test_data = TestData::new();
                let knots = test_data.$knots.values.clone();

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let (start, end, _num_basis) = non_zero_basis(&knots, *u, $order);
                    let basis_funs1 = test_data.$basis.values[idx].clone();

                    let mut basis_funs2 = [0.0; PMAX];
                    eval(&knots, *u, $order, &mut basis_funs2);

                    for i in start..end
                    {
                        let val1 = basis_funs1[i];
                        let val2 = basis_funs2[i - start];
                        assert_relative_eq!(val1, val2, max_relative = 1e-14);
                    }
                }
            }
        };
    }

    eval!(eval0, knots_p0, basis_p0, 0);
    eval!(eval1, knots_p1, basis_p1, 1);
    eval!(eval2, knots_p2, basis_p2, 2);
    eval!(eval3, knots_p3, basis_p3, 3);
    eval!(eval4, knots_p4, basis_p4, 4);
    //..............................................................................................

    macro_rules! eval_diff {
        ($test_name:ident, $knots:ident, $ders:ident, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let p = $order;
                let test_data = TestData::new();
                let knots = test_data.$knots.values.clone();

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let ders_start = idx * (p + 1);
                    let ders_end = (idx + 1) * (p + 1);
                    let ders1_all = test_data.$ders.values[ders_start..ders_end].to_vec();

                    let (start, _end, num_basis) = non_zero_basis(&knots, *u, p);

                    for j in 0..p + 1
                    {
                        let mut ders1 = ders1_all[j].clone();
                        ders1.iter_mut().for_each(|elem| {
                            if elem.abs() < ZERO_THRESHOLD
                            {
                                *elem = 0.0;
                            }
                        });
                        let mut ders2 = [0.0; PMAX + 1];
                        eval_diff(&knots, *u, p, j, &mut ders2);
                        ders2.iter_mut().for_each(|elem| {
                            if elem.abs() < ZERO_THRESHOLD
                            {
                                *elem = 0.0;
                            }
                        });
                        for k in 0..num_basis
                        {
                            let val1 = ders1[k + start];
                            let val2 = ders2[k];
                            assert_relative_eq!(val1, val2, max_relative = 1e-14);
                        }
                    }
                }
            }
        };
    }

    eval_diff!(eval_diff0, knots_p0, ders_p0, 0);
    eval_diff!(eval_diff1, knots_p1, ders_p1, 1);
    eval_diff!(eval_diff2, knots_p2, ders_p2, 2);
    eval_diff!(eval_diff3, knots_p3, ders_p3, 3);
    eval_diff!(eval_diff4, knots_p4, ders_p4, 4);
    //..............................................................................................

    macro_rules! eval_diff_all {
        ($test_name:ident, $knots:ident, $ders:ident, $order:expr) => {
            #[test]
            fn $test_name()
            {
                let p = $order;
                let test_data = TestData::new();
                let knots = test_data.$knots.values.clone();

                for (idx, u) in test_data.u.values.iter().enumerate()
                {
                    let ders_start = idx * (p + 1);
                    let ders_end = (idx + 1) * (p + 1);
                    let mut ders1_all = test_data.$ders.values[ders_start..ders_end].to_vec();
                    ders1_all.iter_mut().for_each(|elem| {
                        elem.iter_mut().for_each(|elem2| {
                            if elem2.abs() < ZERO_THRESHOLD
                            {
                                *elem2 = 0.0;
                            }
                        })
                    });

                    let mut ders2_all = [0.0; (PMAX + 1) * (PMAX + 1)];
                    eval_diff_all(&knots, *u, p, p, &mut ders2_all);
                    ders2_all.iter_mut().for_each(|elem| {
                        if elem.abs() < ZERO_THRESHOLD
                        {
                            *elem = 0.0;
                        }
                    });
                    let ders2_all_arr = NDArrayWrapper::new(&mut ders2_all, &[p + 1, p + 1]);

                    let (start, _end, _num_basis) = non_zero_basis(&knots, *u, p);

                    for i in 0..p + 1
                    {
                        for j in 0..p + 1
                        {
                            let val1 = ders1_all[j][start + i];
                            let val2 = ders2_all_arr[&[i, j]];
                            assert_relative_eq!(val1, val2, max_relative = 1e-14);
                        }
                    }
                }
            }
        };
    }
    eval_diff_all!(eval_diff_all0, knots_p0, ders_p0, 0);
    eval_diff_all!(eval_diff_all1, knots_p1, ders_p1, 1);
    eval_diff_all!(eval_diff_all2, knots_p2, ders_p2, 2);
    eval_diff_all!(eval_diff_all3, knots_p3, ders_p3, 3);
    eval_diff_all!(eval_diff_all4, knots_p4, ders_p4, 4);
    //..............................................................................................

    #[test]
    fn multiplicites_test()
    {
        let u0 = 0.0f64;
        let u1 = 0.2f64;
        let u2_1 = 0.4f64;
        let u2_2 = u2_1.next_up();
        let u2_3 = u2_2.next_up();
        let u3 = 1.0f64;
        let mut u4 = u3; 
        for i in 0..KNOT_ULPS+1
        {
            u4 = u4.next_up();
        }
        let knots = vec![u0, u0, u0, u0, u1, u1, u2_1, u2_2, u2_3, u3, u3, u3, u3, u4];

        let mults1 = vec![(u0, 4), (u1, 2), (u2_1, 3), (u3, 4), (u4, 1)];
        let mults2 = multiplicites(&knots);

        for i in 0..mults1.len()
        {
            assert_eq!(mults1[i].0, mults2[i].0);
            assert_eq!(mults1[i].1, mults2[i].1);
        }
    }
}
