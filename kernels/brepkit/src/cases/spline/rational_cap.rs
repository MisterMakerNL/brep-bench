//! A ball's cap turned from a weighted quadratic arc: the region of the xz plane bounded by the z
//! axis from (0, 0, −5) to (0, 0, 10), the segment from (0, 0, −5) to (5√3, 0, −5), and the
//! rational quadratic Bézier on the case's three control points with weights 1, ½, 1 (brepkit's
//! NURBS), turned a whole turn about the z axis.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{plane_face, rational_bezier, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let s3 = 3f64.sqrt();
    let arc = rational_bezier(
        &[
            p(5.0 * s3, 0.0, -5.0),
            p(10.0 * s3, 0.0, 10.0),
            p(0.0, 0.0, 10.0),
        ],
        &[1.0, 0.5, 1.0],
    )?;
    let section = plane_face(
        topo,
        vec![
            Seg::Line(p(0.0, 0.0, -5.0), p(5.0 * s3, 0.0, -5.0)),
            Seg::Curve(arc),
            Seg::Line(p(0.0, 0.0, 10.0), p(0.0, 0.0, -5.0)),
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
