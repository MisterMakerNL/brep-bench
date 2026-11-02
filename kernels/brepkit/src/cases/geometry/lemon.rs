//! A lemon: the circular segment cut from the disc of radius 5 about (−3, 0, 0) of the xz plane by
//! the z axis (the arc of its circle from (0, 0, −4) out over (2, 0, 0) to (0, 0, 4), closed by the
//! chord along the axis) turned a whole turn about the z axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let (bottom, top) = (p(0.0, 0.0, -4.0), p(0.0, 0.0, 4.0));
    // about −y a turn runs from x towards z: from the bottom out over (2, 0, 0) to the top
    let segment = plane_face(
        topo,
        vec![
            Seg::arc(p(-3.0, 0.0, 0.0), v(0.0, -1.0, 0.0), bottom, top)?,
            Seg::Line(top, bottom),
        ],
    )?;
    Ok(revolve(
        topo,
        segment,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        std::f64::consts::TAU,
    )?)
}
