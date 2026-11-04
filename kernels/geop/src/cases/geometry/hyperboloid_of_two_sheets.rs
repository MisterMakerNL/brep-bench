//! The solid inside the upper sheet of the hyperboloid of two sheets z²/9 − (x² + y²)/16 = 1,
//! below z = 6: the hyperbola's branch of the xz plane from its vertex (0, 3) to (4√3, 6), the
//! line across at z = 6 and the z axis, turned a whole turn about the axis. geop's sketch has no
//! hyperbola: the arc is the rational quadratic Bézier between those ends whose middle control
//! point is where their tangents meet, (4/√3, 3), weighted √(3/2) so that it lies on the
//! hyperbola — exactly the hyperbola.
use crate::shared::surfaces::conic;
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let [vertex, top, rim] =
        [(0.0, 3.0), (0.0, 6.0), (4.0 * 3f64.sqrt(), 6.0)].map(|(x, z)| section.point(zx_at(x, z)));
    let on_axis = section.line(vertex, top);
    section.line(top, rim);
    conic(&mut section, rim, zx_at(4.0 / 3f64.sqrt(), 3.0), vertex, 1.5f64.sqrt());
    sketch(b, "section", zx(), section)?;
    revolve(b, "sheet", "section", sketch_axis("section", on_axis), 360.0, Combine::NewBody)
}
