//! A spheroid: the region in the xz plane between the z axis from (0, 0, −10) to (0, 0, 10) and
//! the rational quadratic B-spline with control points (0, 0, −10), (6, 0, −10), (6, 0, 0),
//! (6, 0, 10), (0, 0, 10), weights 1, √2/2, 1, √2/2, 1 and knots (0, 0, 0, ½, ½, 1, 1, 1),
//! turned a whole turn about the z axis.
//!
//! cadrs's sketch holds no rational curve, so the boundary enters the kernel as a wire
//! (`create_wire`: the B-spline in space and the axis segment drawn on the Front plane), its face
//! is its Fill, and that face is revolved.
use crate::bench::{axis, line, p2, p3, v3, xz};
use crate::shared::spline::{filled, revolve_face};
use crate::shared::sweep::on;
use crate::Refusal;
use cadrs_kernel::{BodyId, Bspline3, DefaultKernel, PathCurve};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let curve = PathCurve::Bspline(Bspline3 {
        degree: 2,
        knots: vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        poles: vec![
            p3(0.0, 0.0, -10.0),
            p3(6.0, 0.0, -10.0),
            p3(6.0, 0.0, 0.0),
            p3(6.0, 0.0, 10.0),
            p3(0.0, 0.0, 10.0),
        ],
        weights: vec![1.0, FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2, 1.0],
    });
    // the Front plane's sketch is the model's (x, z)
    let boundary = [curve, on(xz(0.0), line(p2(0.0, 10.0), p2(0.0, -10.0)))];
    let face = filled(k, &boundary)?;
    revolve_face(k, xz(0.0), face, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)))
}
