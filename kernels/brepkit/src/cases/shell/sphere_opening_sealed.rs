//! The part of the ball of radius 5 about the origin below z = 4.5 (the ball less a box above the
//! plane), shelled inward to a wall 1 thick, open at its flat face. The sphere moved 1 inward, of
//! radius 4, never reaches the plane, so the opening is never reached: the result should be the
//! cut ball less the closed ball of radius 4, its flat face kept.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{ball_below, face_through};
use crate::solids::p;
use crate::Refusal;

const CUT: f64 = 4.5;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cap = ball_below(topo, CUT)?;
    let flat = face_through(topo, cap, p(0.0, 0.0, CUT))?;
    Ok(shell(topo, cap, 1.0, &[flat])?)
}
