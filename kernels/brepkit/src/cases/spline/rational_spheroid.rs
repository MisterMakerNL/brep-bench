//! A spheroid turned from a weighted quadratic B-spline: the region of the xz plane between the z
//! axis from (0, 0, −10) to (0, 0, 10) and the rational quadratic B-spline on the case's five
//! control points, weights and clamped knots (brepkit's NURBS), turned a whole turn about the z
//! axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{nurbs, plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let profile = nurbs(
        2,
        &[0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        &[
            p(0.0, 0.0, -10.0),
            p(6.0, 0.0, -10.0),
            p(6.0, 0.0, 0.0),
            p(6.0, 0.0, 10.0),
            p(0.0, 0.0, 10.0),
        ],
        &[1.0, h, 1.0, h, 1.0],
    )?;
    let section = plane_face(
        topo,
        vec![
            Seg::Curve(profile),
            Seg::Line(p(0.0, 0.0, 10.0), p(0.0, 0.0, -10.0)),
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
