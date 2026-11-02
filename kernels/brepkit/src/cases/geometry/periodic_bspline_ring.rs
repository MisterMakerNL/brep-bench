//! A ring turned from a periodic B-spline: the uniform quadratic B-spline closed on the square of
//! corners (13, 0, 3), (7, 0, 3), (7, 0, −3), (13, 0, −3) in the xz plane (brepkit's NURBS has no
//! periodic flag, so it is written unclamped on evenly spaced knots, its first two control points
//! repeated at its end: the same curve, closing on itself with no end), one closed edge made a flat
//! face and turned a whole turn about the z axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{periodic, plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let corners = [
        p(13.0, 0.0, 3.0),
        p(7.0, 0.0, 3.0),
        p(7.0, 0.0, -3.0),
        p(13.0, 0.0, -3.0),
    ];
    let section = plane_face(topo, vec![Seg::Curve(periodic(2, &corners)?)])?;
    Ok(revolve(
        topo,
        section,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        std::f64::consts::TAU,
    )?)
}
