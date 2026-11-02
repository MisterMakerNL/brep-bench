//! A rod threaded by a ball of radius 1 whose centre follows a helix of radius 5 about the z axis,
//! rising 4 a turn for six turns from (5, 0, −2). brepkit sweeps faces, not solids, so the ball's
//! sweep is built by elementary geometry: a ball moving along a path fills the tube about its
//! centre's path with a ball at each end. The balls at the ends, about (5, 0, −2) and (5, 0, 22),
//! are clear of the rod, so only the tube cuts it: brepkit's helical sweep of a disc of radius 1,
//! turned a quarter turn back about the z axis to start on +x (brepkit's helix starts on +y), cut
//! from the cylinder of radius 5 from z = 0 to z = 20.
use std::f64::consts::FRAC_PI_2;

use brepkit_math::mat::Mat4;
use brepkit_operations::helix::helical_sweep;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::disc;
use crate::solids::{cut, p, placed, v};
use crate::Refusal;

/// NURBS arcs per turn of the helix: how closely brepkit's helix follows the true one.
const SEGMENTS_PER_TURN: usize = 64;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rod = make_cylinder(topo, 5.0, 20.0)?;
    let section = disc(topo, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), 1.0)?;
    let tube = helical_sweep(
        topo,
        section,
        p(0.0, 0.0, -2.0),
        v(0.0, 0.0, 1.0),
        5.0,
        4.0,
        6.0,
        SEGMENTS_PER_TURN,
    )?;
    let tube = placed(topo, tube, Mat4::rotation_z(-FRAC_PI_2))?;
    Ok(cut(topo, rod, tube)?)
}
