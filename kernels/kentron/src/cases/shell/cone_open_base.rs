//! A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8),
//! shelled inward to a wall 1 thick, open at its base. The side, moved 1 inward, is the cone of the
//! same half-angle with its apex at (0, 0, 19/3): the result is the cone less that cone.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cone = Cone::new(6.0, 0.0, 8.0).build(m)?;
    // The cone's edge and apex are convex, where moved faces meet rather than part, so either join
    // gives the same wall; joined by intersection, the apex moves with the side.
    shell::inward(m, cone, &[p(0.0, 0.0, 0.0)], 1.0, Join::Intersection)
}
