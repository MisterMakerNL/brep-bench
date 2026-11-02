//! A pipe of radius 1 along a helix of radius 10 about the z axis, rising 5 a turn for three
//! turns: brepkit's helical sweep of a disc. brepkit's helix starts on +y (its frame's first
//! axis is z × x), so the pipe is turned a quarter turn back about the z axis to start at
//! (10, 0, 0); turning right-handed, it then runs from +x towards +y as described.
use std::f64::consts::FRAC_PI_2;

use brepkit_math::mat::Mat4;
use brepkit_operations::helix::helical_sweep;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::disc;
use crate::solids::{p, placed, v};
use crate::Refusal;

/// NURBS arcs per turn of the helix: how closely brepkit's helix follows the true one.
const SEGMENTS_PER_TURN: usize = 64;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // the section: a disc of radius 1, which the helical sweep sets square to the helix
    let section = disc(topo, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), 1.0)?;
    let pipe = helical_sweep(
        topo,
        section,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        10.0,
        5.0,
        3.0,
        SEGMENTS_PER_TURN,
    )?;
    Ok(placed(topo, pipe, Mat4::rotation_z(-FRAC_PI_2))?)
}
