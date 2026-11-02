//! A spindle torus about the z axis, middle circle radius 3 and tube radius 5, counted once: the
//! part of the tube's disc (x − 3)² + z² ≤ 25 with x ≥ 0 (the arc of the tube's circle about
//! (3, 0, 0) from (0, 0, −4) out over (8, 0, 0) to (0, 0, 4), closed by the segment of the axis)
//! turned a whole turn about the z axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let (bottom, top) = (p(0.0, 0.0, -4.0), p(0.0, 0.0, 4.0));
    // about −y a turn runs from x towards z: from the bottom out over (8, 0, 0) to the top
    let section = plane_face(
        topo,
        vec![
            Seg::arc(p(3.0, 0.0, 0.0), v(0.0, -1.0, 0.0), bottom, top)?,
            Seg::Line(top, bottom),
        ],
    )?;
    Ok(revolve(
        topo,
        section,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        std::f64::consts::TAU,
    )?)
}
