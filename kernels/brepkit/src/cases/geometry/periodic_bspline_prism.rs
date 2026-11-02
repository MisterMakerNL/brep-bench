//! A prism on a periodic B-spline: the uniform quadratic B-spline closed on the square of corners
//! (10, 10), (−10, 10), (−10, −10), (10, −10) in the plane z = 0 (brepkit's NURBS has no periodic
//! flag, so it is written unclamped on evenly spaced knots, its first two control points repeated
//! at its end: the same curve, closing on itself with no end), one closed edge made a flat face and
//! extruded straight up by 10.
use brepkit_operations::extrude::extrude;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{periodic, plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let corners = [
        p(10.0, 10.0, 0.0),
        p(-10.0, 10.0, 0.0),
        p(-10.0, -10.0, 0.0),
        p(10.0, -10.0, 0.0),
    ];
    let section = plane_face(topo, vec![Seg::Curve(periodic(2, &corners)?)])?;
    Ok(extrude(topo, section, v(0.0, 0.0, 1.0), 10.0)?)
}
