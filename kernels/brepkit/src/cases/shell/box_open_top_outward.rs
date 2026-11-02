//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top, the
//! wall every point outside the box within 1 of its other faces: its outside rounded, a quarter
//! cylinder of radius 1 about each of the box's edges below the rim, an eighth of a ball at each
//! bottom corner.
//!
//! brepkit's shells join the moved faces only sharp (its arc joint is not implemented), so the
//! wall is grown sharp with `shell_v2` (a positive thickness grows it outside the faces), and the
//! outside's eight edges below the rim are rounded to radius 1 with `fillet_v2`. Each sharp edge
//! is 1 from both of its moved faces, so its round of radius 1 is the quarter cylinder about the
//! box's own edge; where three rounds meet at a bottom corner, the corner patch is the ball of
//! radius 1 about the box's corner.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::shell_v2;
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::hollow::face_through;
use crate::solids::{edges_where, near, p};
use crate::Refusal;

const T: f64 = 1.0;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 8.0, 6.0)?;
    let top = face_through(topo, block, p(5.0, 4.0, 6.0))?;
    let cup = shell_v2(topo, block, T, &[top])?;
    // the outside's edges: on two of the moved faces' planes x = -1 or 11, y = -1 or 9, z = -1
    let outside = |c: f64, lo: f64, hi: f64| near(c, lo - T) || near(c, hi + T);
    let edges = edges_where(topo, cup, |q| {
        let on = [
            outside(q.x(), 0.0, 10.0),
            outside(q.y(), 0.0, 8.0),
            near(q.z(), -T),
        ];
        on.iter().filter(|&&b| b).count() >= 2
    })?;
    Ok(all_blended(fillet_v2(topo, cup, &edges, T)?)?)
}
