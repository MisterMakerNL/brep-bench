//! A hyperboloid of one sheet: the region of the xz plane between the branch x > 0 of the
//! hyperbola x²/16 − z²/36 = 1 (brepkit's hyperbola about the origin, real semi-axis 4 along x and
//! imaginary 6 along z), the lines z = ±6 and the z axis, turned a whole turn about the z axis.
//! brepkit's edges carry no hyperbola, so its arc from z = −6 to z = 6 (the parameter ±asinh 1) is
//! the exact rational quadratic Bézier on it.
use brepkit_math::curves::Hyperbola3D;
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::curved::{hyperbola_arc, plane_face, Seg};
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // H(t) = (4 cosh t, 0, 6 sinh t)
    let hyperbola = Hyperbola3D::with_axes(
        p(0.0, 0.0, 0.0),
        4.0,
        6.0,
        v(1.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
    )?;
    let end = 1f64.asinh();
    let arc = hyperbola_arc(&hyperbola, -end, end)?;
    let (low, high) = (hyperbola.evaluate(-end), hyperbola.evaluate(end));
    let section = plane_face(
        topo,
        vec![
            Seg::Curve(arc),
            Seg::Line(high, p(0.0, 0.0, 6.0)),
            Seg::Line(p(0.0, 0.0, 6.0), p(0.0, 0.0, -6.0)),
            Seg::Line(p(0.0, 0.0, -6.0), low),
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
