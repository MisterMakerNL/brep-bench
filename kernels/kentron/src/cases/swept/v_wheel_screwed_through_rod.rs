//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove
//! ground by a wheel with a sharp 60° V rim carried along a screw. The wheel: the solid of
//! revolution, about the axis through (14, 0, 0) along (0, cos h, −sin h) with
//! h = atan(2π·4.5 / 30), of the triangle with its tip at (4, 0, 0) and its other corners 2.5
//! further out, ±2.5·tan 30° along the axis. The screw: about the z axis, right-handed, rising 30
//! a turn, from a turn of −144° to +384°. Everything the wheel passes through, cut.
//!
//! Refused. kentron's one sweep that can follow a screw is a pipe along a helix in its Frenet
//! frame (`sweep::pipe_with`, `frenet`), and it takes no solid ("a pipe of a Solid"). Only the
//! sharp rim's sweep is a helicoid of the wheel's own edge; each conical flank meets the groove
//! along the cone's characteristic curve, where the screw runs along the cone, none of the wheel's
//! edges. The groove's flanks are the envelopes those curves sweep out, which kentron has no
//! operation to find, and computing them here would be making the geometry outside the kernel.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a screw motion: sweep::pipe along a helix takes no \
         solid, and the groove's flanks are the envelopes of the screwed cones, along characteristic \
         curves that are none of the wheel's edges, which kentron has no operation to find",
    ))
}
