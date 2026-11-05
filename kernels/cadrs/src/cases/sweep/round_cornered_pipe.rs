//! A pipe of radius 3 along two straight runs, (−30, 0, 0) to the origin and on along +y to
//! (0, 30, 0), the section carried around the corner by turning it about the z axis.
//!
//! cadrs's sweep mitres a corner of its path and has no round corner join, so the pipe is built
//! as the description gives it: each run the disk swept straight along it (a circle extruded,
//! ending flat at the corner), and the disk turned about the z axis a quarter turn from square
//! to the first run to square to the second, which fills the wedge outside the corner (its half
//! on the outside, y ≤ 0, revolved; the inside half turns within the runs).
use crate::bench::{arc, axis, chain, cylinder, line, p2, p3, revolve, sketch, union_each, v3, yz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let first = cylinder(k, p3(-30.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), 3.0, 30.0)?;
    let second = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), 3.0, 30.0)?;
    // the half of the disk on the Right plane (sketch (y, z)) with y ≤ 0
    let half = sketch(
        yz(0.0),
        vec![chain(vec![arc(p2(0.0, 0.0), 3.0, 90.0, 180.0), line(p2(0.0, -3.0), p2(0.0, 3.0))])],
    );
    let corner = revolve(k, &half, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), Some(90.0))?;
    union_each(k, first, &[corner, second])
}
