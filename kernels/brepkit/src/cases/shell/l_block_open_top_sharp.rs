//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, shelled inward to a wall 1 thick, open at its top, every pair of moved faces extended
//! until they meet: brepkit's `shell` joins its moved faces that way, so at the concave edge the
//! moved walls x = 4 and y = 4 meet in the line x = 4, y = 4.
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{face_through, plate};
use crate::solids::p;
use crate::Refusal;

const OUTLINE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (20.0, 0.0),
    (20.0, 5.0),
    (5.0, 5.0),
    (5.0, 15.0),
    (0.0, 15.0),
];

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let l = plate(topo, &OUTLINE, 8.0)?;
    let top = face_through(topo, l, p(2.0, 2.0, 8.0))?;
    Ok(shell(topo, l, 1.0, &[top])?)
}
