use crate::common::Vector;

pub const ZERO_THRESHOLD: f64 = 1e-13;



pub fn convert<const D: usize>(data: &Vec<Vec<f64>>) -> Vec<Vector<D>>
{
    for val in data
    {
        assert!(val.len() == D);
    }

    // let mut out = Vec::<Vector<D>>::with_capacity(data.len());
    let mut out = vec![Vector::<D>::zeros(); data.len()];

    for (idx, val) in data.iter().enumerate()
    {
        out[idx].copy_from_slice(val);
    }
    out
}

pub fn de_noise(data: &mut [f64])
{
    data.iter_mut().for_each(|elem| {
        if elem.abs() < ZERO_THRESHOLD
        {
            *elem = 0.0;
        }
    })
}