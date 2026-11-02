//! A pipe of radius 1 along a helix of radius 10 about the z axis, rising 5 a turn for three
//! turns, each end closed by a half ball: brepkit's helical sweep of a disc, fused with a ball of
//! radius 1 about each end of the helix, (10, 0, 0) and (10, 0, 15). brepkit's helix starts on
//! +y (its frame's first axis is z × x), so the pipe is turned a quarter turn back about the
//! z axis to start at (10, 0, 0).
use std::f64::consts::FRAC_PI_2;

use brepkit_math::mat::Mat4;
use brepkit_operations::helix::helical_sweep;
use brepkit_operations::primitives::make_sphere;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::disc;
use crate::solids::{fuse, moved, p, placed, v};
use crate::Refusal;

/// NURBS arcs per turn of the helix: how closely brepkit's helix follows the true one.
const SEGMENTS_PER_TURN: usize = 64;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
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
    let pipe = placed(topo, pipe, Mat4::rotation_z(-FRAC_PI_2))?;
    // three whole turns end straight above the start, 15 up
    let ball = make_sphere(topo, 1.0, 64)?;
    let start = moved(topo, ball, 10.0, 0.0, 0.0)?;
    let ball = make_sphere(topo, 1.0, 64)?;
    let end = moved(topo, ball, 10.0, 0.0, 15.0)?;
    let pipe = fuse(topo, pipe, start)?;
    Ok(fuse(topo, pipe, end)?)
}
