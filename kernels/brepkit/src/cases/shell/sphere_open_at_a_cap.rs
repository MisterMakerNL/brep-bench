//! The part of the ball of radius 5 about the origin below z = 3 (the ball less a box above the
//! plane), shelled inward to a wall 1 thick, open at its flat face: brepkit's `shell` moves the
//! sphere to the sphere of radius 4, which runs on to the plane z = 3, the rim the annulus between
//! radii √7 and 4 there.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{ball_below, face_through};
use crate::solids::p;
use crate::Refusal;

const CUT: f64 = 3.0;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cap = ball_below(topo, CUT)?;
    let flat = face_through(topo, cap, p(0.0, 0.0, CUT))?;
    Ok(shell(topo, cap, 1.0, &[flat])?)
}
