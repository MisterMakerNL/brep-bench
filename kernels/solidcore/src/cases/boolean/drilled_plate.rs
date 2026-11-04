//! A plate from the origin to (100, 20, 10), drilled through along z by nine holes of radius 2
//! at y = 10 and x = 10, 20, ... 90, one cut after another, each drill running past both faces.
use crate::bench::cut_all;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([0.0, 0.0, 0.0], [100.0, 20.0, 10.0])?;
    let drills = (1..=9)
        .map(|i| make_cylinder([10.0 * f64::from(i), 10.0, -1.0], 2.0, 12.0))
        .collect::<solidcore_ops::Result<Vec<_>>>()?;
    cut_all(plate, drills)
}
