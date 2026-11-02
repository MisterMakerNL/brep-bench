//! A dome turned from a cubic Bézier curve: the region of the xz plane bounded by the z axis from
//! the origin to (0, 0, 10), the x axis from the origin to (9, 0, 0), and the cubic Bézier on the
//! case's four control points (brepkit's NURBS), turned a whole turn about the z axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{bezier, plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let profile = bezier(&[
        p(9.0, 0.0, 0.0),
        p(12.0, 0.0, 4.0),
        p(9.0, 0.0, 10.0),
        p(0.0, 0.0, 10.0),
    ])?;
    let section = plane_face(
        topo,
        vec![
            Seg::Line(p(0.0, 0.0, 0.0), p(9.0, 0.0, 0.0)),
            Seg::Curve(profile),
            Seg::Line(p(0.0, 0.0, 10.0), p(0.0, 0.0, 0.0)),
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
