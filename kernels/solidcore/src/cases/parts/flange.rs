//! A round mounting flange on the z axis: a plate (a disc of radius 32, z from 0 to 6) and a hub
//! (radius 15, z from 6 to 16) united, bored through by radius 8 on the axis, and six bolt holes
//! of radius 3 through the plate on the circle of radius 24, one every 60°: each hole a cylinder
//! running past both faces of the plate, cut.
use crate::bench::{cut, cut_all, fuse};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};
use std::f64::consts::PI;

pub fn build() -> Result<Body, Refusal> {
    let plate = make_cylinder([0.0; 3], 32.0, 6.0)?;
    let hub = make_cylinder([0.0, 0.0, 6.0], 15.0, 10.0)?;
    let bore = make_cylinder([0.0, 0.0, -1.0], 8.0, 18.0)?;
    let flange = cut(fuse(plate, hub)?, bore)?;
    let holes = (0..6)
        .map(|i| {
            let a = i as f64 * PI / 3.0;
            make_cylinder([24.0 * a.cos(), 24.0 * a.sin(), -1.0], 3.0, 8.0)
        })
        .collect::<Result<Vec<_>, _>>()?;
    cut_all(flange, holes)
}
