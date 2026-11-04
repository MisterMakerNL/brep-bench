//! The solid inside the hyperboloid of one sheet x² + y² ≤ 16(1 + z²/36), −6 ≤ z ≤ 6: the
//! hyperbola x²/16 − z²/36 = 1 of the xz plane, from (4√2, −6) through its vertex (4, 0) to
//! (4√2, 6), and the z axis, closed by the lines across at z = ±6, turned a whole turn about the
//! axis. geop's sketch has no hyperbola: the arc is the rational quadratic Bézier between those
//! ends whose middle control point is where their tangents meet, (2√2, 0), weighted √2 so that
//! it passes through the vertex — exactly the hyperbola.
use crate::shared::surfaces::conic;
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let r = 4.0 * 2f64.sqrt();
    let mut section = Drawing::new();
    let [bottom, top, top_rim, bottom_rim] =
        [(0.0, -6.0), (0.0, 6.0), (r, 6.0), (r, -6.0)].map(|(x, z)| section.point(zx_at(x, z)));
    let on_axis = section.line(bottom, top);
    section.line(top, top_rim);
    conic(&mut section, top_rim, zx_at(2.0 * 2f64.sqrt(), 0.0), bottom_rim, 2f64.sqrt());
    section.line(bottom_rim, bottom);
    sketch(b, "section", zx(), section)?;
    revolve(b, "hyperboloid", "section", sketch_axis("section", on_axis), 360.0, Combine::NewBody)
}
