//! A torus about the z axis of radii 5 and 3, split by the plane 3x − 4z = 0 through the origin,
//! which touches it at two points; the half on and below the plane (z ≤ 3x/4) is kept, the side
//! away from the plane's normal (−3, 0, 4). brepkit's torus has its seam meridian on +x, so it is
//! first turned a quarter turn about z to put the seam in the plane x = 0 on the side y > 0, away
//! from the tangent points, as the case asks.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::make_torus;
use brepkit_operations::split::split;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::FRAC_PI_2;

use crate::solids::{p, placed, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ring = make_torus(topo, 5.0, 3.0, 64)?;
    let ring = placed(topo, ring, Mat4::rotation_z(FRAC_PI_2))?;
    let halves = split(topo, ring, p(0.0, 0.0, 0.0), v(-3.0, 0.0, 4.0))?;
    Ok(halves.negative)
}
