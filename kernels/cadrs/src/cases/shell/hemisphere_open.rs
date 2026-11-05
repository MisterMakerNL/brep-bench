//! A half ball shelled open at its flat face: a bowl. The lower half of the ball of radius 5 centered
//! on the origin, shelled inward to a wall 1 thick, open at its flat face, z = 0: the half ball less
//! the half ball of radius 4.
//!
//! The half ball is a quarter disc beside the z axis revolved a whole turn (cadrs has no sphere
//! primitive); cadrs's Shell feature (inward), the flat face picked as the face to remove.
use crate::bench::{arc, axis, chain, line, p2, p3, revolve, sketch, through_axis, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let quarter = chain(vec![
        line(p2(0.0, 0.0), p2(5.0, 0.0)),
        arc(p2(0.0, 0.0), 5.0, 0.0, -90.0),
        line(p2(0.0, -5.0), p2(0.0, 0.0)),
    ]);
    let s = sketch(through_axis(p3(0.0, 0.0, 0.0), V3::z()), vec![quarter]);
    let half = revolve(k, &s, axis(p3(0.0, 0.0, 0.0), V3::z()), None)?;
    shell_inward(k, half, &[p3(2.0, 1.0, 0.0)], 1.0, Joins::Rounded)
}
