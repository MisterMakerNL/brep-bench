//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, shelled inward to a wall 1 thick, open at its top, the wall every point of the plate
//! within 1 of its faces other than the top: at the concave edge x = 5, y = 5 its inside is the
//! quarter cylinder of radius 1 about the edge.
//!
//! brepkit's shells join the moved faces only sharp (its arc joint is not implemented): `shell`
//! makes the moved walls x = 4 and y = 4 meet in the line x = 4, y = 4, a convex edge of the
//! wall's inside. Rounded to radius 1 with `fillet_v2`, that edge's round is tangent to x = 4 and
//! y = 4 about the line x = 5, y = 5: the quarter cylinder about the concave edge.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::shell_op::shell;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::{face_through, plate};
use crate::solids::{edges_where, near, p};
use crate::Refusal;

const OUTLINE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (20.0, 0.0),
    (20.0, 5.0),
    (5.0, 5.0),
    (5.0, 15.0),
    (0.0, 15.0),
];
const T: f64 = 1.0;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let l = plate(topo, &OUTLINE, 8.0)?;
    let top = face_through(topo, l, p(2.0, 2.0, 8.0))?;
    let cup = shell(topo, l, T, &[top])?;
    // the inside's edge where the moved walls meet: the vertical line x = y = 5 - T
    let corner = edges_where(topo, cup, |q| near(q.x(), 5.0 - T) && near(q.y(), 5.0 - T))?;
    Ok(all_blended(fillet_v2(topo, cup, &corner, T)?)?)
}
