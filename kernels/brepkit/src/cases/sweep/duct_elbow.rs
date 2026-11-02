//! A hollow square duct bent a quarter turn: the section, the square 18 × 18 with a square hole
//! 14 × 14, standing in the xz plane centred at (30, 0, 0), revolved a quarter turn about the
//! z axis from +x towards +y, which is the same section carried all the way round the arc.
use std::f64::consts::FRAC_PI_2;

use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::holed;
use crate::solids::{p, v};
use crate::Refusal;

/// The square of half-width `h` in the xz plane, centred at (30, 0, 0).
fn square(h: f64) -> Vec<brepkit_math::vec::Point3> {
    vec![
        p(30.0 - h, 0.0, -h),
        p(30.0 + h, 0.0, -h),
        p(30.0 + h, 0.0, h),
        p(30.0 - h, 0.0, h),
    ]
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let section = holed(topo, &square(9.0), &square(7.0))?;
    Ok(revolve(
        topo,
        section,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        FRAC_PI_2,
    )?)
}
