//! A solid spindle torus centered on the origin, about the z axis: the circle through the middle
//! of its tube has radius 3 and the tube has radius 5, so the tube crosses the axis. The solid is
//! what the disk (x − 3)² + z² ≤ 25 fills as it turns, counted once: the turn of the disk's part
//! with x ≥ 0, bounded by the tube circle's outer arc, which meets the axis at (0, 0, ±4).
//!
//! On the Front (xz) plane, that part of the disk: the arc of the circle of radius 5 about
//! (3, 0) from (0, −4) round through (8, 0) to (0, 4), closed by the z axis, revolved a whole turn
//! about z (cadrs has no torus primitive).
use crate::bench::{arc, axis, chain, line, p2, p3, revolve, sketch, v3, xz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // The arc's ends (0, ±4) lie at angles ±(180° − arccos(3/5)) about (3, 0).
    let end = 180.0 - (3.0f64 / 5.0).acos().to_degrees();
    let apple = chain(vec![
        arc(p2(3.0, 0.0), 5.0, -end, 2.0 * end),
        line(p2(0.0, 4.0), p2(0.0, -4.0)),
    ]);
    let s = sketch(xz(0.0), vec![apple]);
    revolve(k, &s, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)
}
