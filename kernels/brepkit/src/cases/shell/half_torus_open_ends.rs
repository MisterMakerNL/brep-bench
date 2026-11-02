//! The half with y >= 0 of the solid torus about the z axis whose tube has radius 2 about a middle
//! circle of radius 6 (the torus less a box covering y < 0), shelled inward to a wall 0.5 thick,
//! open at both flat ends: brepkit's `shell` moves the torus face to the torus of tube radius 1.5,
//! a curved pipe with an annular rim at each end.
use brepkit_operations::primitives::{make_box, make_torus};
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::faces_through;
use crate::solids::{cut, moved, p};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let torus = make_torus(topo, 6.0, 2.0, 64)?;
    let below = make_box(topo, 20.0, 10.0, 6.0)?;
    let below = moved(topo, below, -10.0, -10.0, -3.0)?;
    let half = cut(topo, torus, below)?;
    let ends = faces_through(topo, half, &[p(6.0, 0.0, 0.0), p(-6.0, 0.0, 0.0)])?;
    Ok(shell(topo, half, 0.5, &ends)?)
}
