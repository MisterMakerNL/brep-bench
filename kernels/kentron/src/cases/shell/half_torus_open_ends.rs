//! The half with y ≥ 0 of the solid torus about the z axis, centred on the origin, its tube of
//! radius 2 about a middle circle of radius 6, shelled inward to a wall 0.5 thick, open at both
//! flat end discs in the plane y = 0: a curved pipe, the half torus less the half torus of tube
//! radius 1.5.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // revolved half a turn from the x axis towards +y: the half with y ≥ 0
    let half = Torus::new(6.0, 2.0).with_angle(PI).build(m)?;
    // open at both ends, the discs about (6, 0, 0) and (-6, 0, 0)
    shell::inward(m, half, &[p(6.0, 0.0, 0.0), p(-6.0, 0.0, 0.0)], 0.5, Join::Arc)
}
