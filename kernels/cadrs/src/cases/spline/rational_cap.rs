//! A ball's cap: the region in the xz plane bounded by the z axis from (0, 0, −5) to (0, 0, 10),
//! the segment from (0, 0, −5) to (5√3, 0, −5), and the rational quadratic Bézier curve from
//! (5√3, 0, −5) through the control point (10√3, 0, 10) to (0, 0, 10), weights 1, ½ and 1,
//! turned a whole turn about the z axis.
//!
//! cadrs's sketch holds no rational curve, so the boundary enters the kernel as a wire
//! (`create_wire`: the two segments drawn on the Front plane and the weighted Bézier curve in
//! space), its face is its Fill, and that face is revolved.
use crate::bench::{axis, line, p2, p3, v3, xz};
use crate::shared::spline::{filled, revolve_face};
use crate::shared::sweep::on;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, PathCurve};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let r = 5.0 * 3f64.sqrt();
    let arc = PathCurve::Bezier3 {
        poles: vec![p3(r, 0.0, -5.0), p3(2.0 * r, 0.0, 10.0), p3(0.0, 0.0, 10.0)],
        weights: vec![1.0, 0.5, 1.0],
    };
    // the Front plane's sketch is the model's (x, z)
    let boundary = [
        on(xz(0.0), line(p2(0.0, 10.0), p2(0.0, -5.0))),
        on(xz(0.0), line(p2(0.0, -5.0), p2(r, -5.0))),
        arc,
    ];
    let face = filled(k, &boundary)?;
    revolve_face(k, xz(0.0), face, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)))
}
