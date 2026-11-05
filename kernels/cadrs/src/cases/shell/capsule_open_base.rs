//! A capsule's end shelled open at its base, its walls meeting tangentially. A cylinder of radius 5 on
//! the z axis from z = 0 to z = 10, capped by the half ball of radius 5 about (0, 0, 10), shelled
//! inward to a wall 1 thick, open at its base, z = 0: the capsule less the capsule of radius 4.
//!
//! The capsule's end is its section revolved (cadrs has no sphere primitive to join to a cylinder);
//! cadrs's Shell feature (inward), the base picked as the face to remove.
use crate::bench::{arc, axis, chain, line, p2, p3, revolve, sketch, through_axis, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

/// The capsule's end: its section beside the z axis (the side's line, the dome's quarter arc about
/// (0, 0, 10)) revolved a whole turn.
fn capsule_end(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = chain(vec![
        line(p2(0.0, 0.0), p2(5.0, 0.0)),
        line(p2(5.0, 0.0), p2(5.0, 10.0)),
        arc(p2(0.0, 10.0), 5.0, 0.0, 90.0),
        line(p2(0.0, 15.0), p2(0.0, 0.0)),
    ]);
    let s = sketch(through_axis(p3(0.0, 0.0, 0.0), V3::z()), vec![section]);
    revolve(k, &s, axis(p3(0.0, 0.0, 0.0), V3::z()), None)
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = capsule_end(k)?;
    shell_inward(k, c, &[p3(2.0, 1.0, 0.0)], 1.0, Joins::Rounded)
}
