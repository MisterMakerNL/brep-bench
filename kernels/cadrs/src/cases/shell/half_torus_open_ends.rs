//! A half torus shelled open at both ends: a curved pipe. The half with y ≥ 0 of the solid torus about
//! the z axis whose tube has radius 2 about a middle circle of radius 6, shelled inward to a wall 0.5
//! thick, open at both flat faces (the disks about (6, 0, 0) and (−6, 0, 0) in y = 0): the half torus
//! less the half torus of tube radius 1.5.
//!
//! The half torus is the tube's circle, on a plane through the z axis, revolved half a turn (cadrs
//! has no torus primitive); cadrs's Shell feature (inward), the two end disks picked as the faces to
//! remove.
use crate::bench::{axis, p3, revolve, round, sketch, through_axis, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the plane through the z axis with its sketch x along the model's x: turned +180° about z the
    // circle sweeps through y > 0
    let s = sketch(through_axis(p3(0.0, 0.0, 0.0), V3::z()), vec![round((6.0, 0.0), 2.0)]);
    let half = revolve(k, &s, axis(p3(0.0, 0.0, 0.0), V3::z()), Some(180.0))?;
    shell_inward(k, half, &[p3(6.0, 0.0, 0.5), p3(-6.0, 0.0, 0.5)], 0.5, Joins::Rounded)
}
