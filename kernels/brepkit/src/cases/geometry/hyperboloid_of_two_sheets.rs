//! A cap of a hyperboloid of two sheets: the region of the xz plane between the upper branch of the
//! hyperbola z²/9 − x²/16 = 1 (brepkit's hyperbola about the origin, real semi-axis 3 along z and
//! imaginary 4 along x), the line z = 6 and the z axis, turned a whole turn about the z axis.
//! brepkit's edges carry no hyperbola, so its arc from the vertex (0, 0, 3) to (4√3, 0, 6) (the
//! parameter 0 to acosh 2) is brepkit-heal's exact NURBS of it.
use brepkit_math::curves::Hyperbola3D;
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use brepkit_heal::construct::convert_curve::hyperbola_to_nurbs;
use crate::solids::curved::{plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // H(t) = (4 sinh t, 0, 3 cosh t)
    let hyperbola = Hyperbola3D::with_axes(
        p(0.0, 0.0, 0.0),
        3.0,
        4.0,
        v(0.0, 0.0, 1.0),
        v(1.0, 0.0, 0.0),
    )?;
    let end = 2f64.acosh();
    let arc = hyperbola_to_nurbs(&hyperbola, 0.0, end)?;
    let rim = hyperbola.evaluate(end);
    let section = plane_face(
        topo,
        vec![
            Seg::Curve(arc),
            Seg::Line(rim, p(0.0, 0.0, 6.0)),
            Seg::Line(p(0.0, 0.0, 6.0), p(0.0, 0.0, 3.0)),
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
