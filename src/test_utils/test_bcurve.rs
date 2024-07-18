//! This submodule provides data and utilities for testing B-Curves
//!
//--------------------------------------------------------------------------------------------------

use serde::Deserialize;

use std::fs;

use crate::geometry::{Bcurve, BcurveDescriptor, BCURVE_DER_MAX};

use super::convert;



#[derive(Deserialize)]
pub struct ParamData
{
    pub description: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct KnotData
{
    pub description: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct WeightData
{
    pub description: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct CpointData
{
    pub description: String,
    pub values: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
pub struct PointData
{
    pub description: String,
    pub values: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
pub struct DerData
{
    pub description: String,
    pub values: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
pub struct TangentData
{
    pub description: String,
    pub values: Vec<Vec<f64>>,
}

#[derive(Deserialize)]
pub struct TestData
{
    pub u: ParamData,
    pub knots_p1: KnotData,
    pub knots_p2: KnotData,
    pub knots_p3: KnotData,
    pub knots_p4: KnotData,
    pub weights_p1: WeightData,
    pub weights_p2: WeightData,
    pub weights_p3: WeightData,
    pub weights_p4: WeightData,
    pub cpoints_d2_p1: CpointData,
    pub cpoints_d2_p2: CpointData,
    pub cpoints_d2_p3: CpointData,
    pub cpoints_d2_p4: CpointData,
    pub cpoints_d3_p1: CpointData,
    pub cpoints_d3_p2: CpointData,
    pub cpoints_d3_p3: CpointData,
    pub cpoints_d3_p4: CpointData,
    pub points_d2_p1: PointData,
    pub points_d2_p2: PointData,
    pub points_d2_p3: PointData,
    pub points_d2_p4: PointData,
    pub points_d3_p1: PointData,
    pub points_d3_p2: PointData,
    pub points_d3_p3: PointData,
    pub points_d3_p4: PointData,
    pub ders_d2_p1: DerData,
    pub ders_d2_p2: DerData,
    pub ders_d2_p3: DerData,
    pub ders_d2_p4: DerData,
    pub ders_d3_p1: DerData,
    pub ders_d3_p2: DerData,
    pub ders_d3_p3: DerData,
    pub ders_d3_p4: DerData,
    pub tangent_d2_p1: TangentData,
    pub tangent_d2_p2: TangentData,
    pub tangent_d2_p3: TangentData,
    pub tangent_d2_p4: TangentData,
    pub tangent_d3_p1: TangentData,
    pub tangent_d3_p2: TangentData,
    pub tangent_d3_p3: TangentData,
    pub tangent_d3_p4: TangentData,
}

impl TestData
{
    pub fn new() -> Self
    {
        let json_file = fs::read_to_string("assets/geo/bcurve-tests.json").expect("Unable to read file");
        serde_json::from_str(&json_file).expect("Could not deserialize")
    }
}


pub fn load_bcurve<const D: usize>(p: usize, test_data: &TestData) -> Bcurve<D>
where
    [(); D + 1]:,
    [(); D * BCURVE_DER_MAX]:,
    [(); D * 3]:,
{
    let test_data = TestData::new();

    let knots = match p 
    {
        1 => test_data.knots_p1.values.clone(),
        2 => test_data.knots_p2.values.clone(),
        3 => test_data.knots_p3.values.clone(),
        4 => test_data.knots_p4.values.clone(),
        _ => panic!("Invalid value for p: {}", p),
    };


    let weights = match p 
    {
        1 => test_data.weights_p1.values.clone(),
        2 => test_data.weights_p2.values.clone(),
        3 => test_data.weights_p3.values.clone(),
        4 => test_data.weights_p4.values.clone(),
        _ => panic!("Invalid value for p: {}", p),
    };

    let cpoints = if D == 2
    {
        match p {
            1 => convert(&test_data.cpoints_d2_p1.values),
            2 => convert(&test_data.cpoints_d2_p2.values),
            3 => convert(&test_data.cpoints_d2_p3.values),
            4 => convert(&test_data.cpoints_d2_p4.values),
            _ => panic!("Invalid value for p: {}", p),
        }
    }
    else if D == 3
    {
        match p {
            1 => convert(&test_data.cpoints_d3_p1.values),
            2 => convert(&test_data.cpoints_d3_p2.values),
            3 => convert(&test_data.cpoints_d3_p3.values),
            4 => convert(&test_data.cpoints_d3_p4.values),
            _ => panic!("Invalid value for p: {}", p),
        }

    }
    else 
    {
        panic!("D must be either 2 or 3");
    };
    let bcurve_descriptor = BcurveDescriptor {
        p: p,
        knots: knots,
        cpoints: cpoints,
        cweights: weights,
    };
    let bcurve = Bcurve::<D>::new(&bcurve_descriptor);
    bcurve
}
