//! A paraboloid of revolution: the region of the xz plane between the parabola x² = 8z (brepkit's
//! parabola with its vertex at the origin, opening up the z axis, focal length 2), the line z = 8
//! and the z axis, turned a whole turn about the z axis. brepkit's edges carry no parabola, so its
//! arc from the vertex to (8, 0, 8) is brepkit-heal's exact NURBS of it.
use brepkit_math::curves::Parabola3D;
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use brepkit_heal::construct::convert_curve::parabola_to_nurbs;
use crate::solids::curved::{plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // P(t) = (t, 0, t²/8): t runs along x from the vertex
    let parabola =
        Parabola3D::with_axes(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0), 2.0)?;
    let section = plane_face(
        topo,
        vec![
            Seg::Curve(parabola_to_nurbs(&parabola, 0.0, 8.0)?),
            Seg::Line(p(8.0, 0.0, 8.0), p(0.0, 0.0, 8.0)),
            Seg::Line(p(0.0, 0.0, 8.0), p(0.0, 0.0, 0.0)),
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
