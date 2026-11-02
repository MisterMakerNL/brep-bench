//! A peanut prism: the uniform periodic cubic B-spline on the case's twelve control points in the
//! plane z = 0 (brepkit's NURBS has no periodic flag, so it is written unclamped on evenly spaced
//! knots, its first three control points repeated at its end: the same curve, C² all round with no
//! end), one closed edge made a flat face and extruded straight up by 10.
use brepkit_operations::extrude::extrude;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{periodic, plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let points = [
        (14.0, 0.0),
        (12.0, 8.0),
        (4.0, 6.0),
        (0.0, -21.0 / 8.0),
        (-4.0, 6.0),
        (-12.0, 8.0),
        (-14.0, 0.0),
        (-12.0, -8.0),
        (-4.0, -6.0),
        (0.0, 21.0 / 8.0),
        (4.0, -6.0),
        (12.0, -8.0),
    ]
    .map(|(x, y)| p(x, y, 0.0));
    let section = plane_face(topo, vec![Seg::Curve(periodic(3, &points)?)])?;
    Ok(extrude(topo, section, v(0.0, 0.0, 1.0), 10.0)?)
}
