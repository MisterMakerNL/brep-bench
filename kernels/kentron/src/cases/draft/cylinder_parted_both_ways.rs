//! A cylinder of radius 10 about the z axis from z = 0 to z = 20, its side split at the parting
//! line, the circle at z = 10, and each half drafted 3° away from it about the neutral plane
//! z = 10: the upper half with the pull direction +z, the lower with −z, each leaning inward as
//! it leaves the parting line. Two frustums of cones joined at their wide ends.
//!
//! kentron's draft drafts every face tangent to a drafted one along with it, and the two halves
//! of the side meet tangentially at the parting line, so on the one cylinder either draft would
//! take both halves. Each half is therefore drafted as its own solid, the cylinder on its side of
//! the parting line, and the two joined there.
use crate::bench::{face_through, frame, p};
use crate::shared::draft::drafted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let neutral = p(0.0, 0.0, 10.0);
    let lower = Cylinder::new(10.0, 10.0).build(m)?;
    let upper = Cylinder::new(10.0, 10.0).with_frame(frame(neutral, Direction::Z)).build(m)?;
    // Each half pulled away from the parting line.
    let upper_side = face_through(m, upper, p(0.0, 10.0, 15.0))?;
    let upper = drafted(m, upper, &[upper_side], 3.0, neutral, Direction::Z)?;
    let lower_side = face_through(m, lower, p(0.0, 10.0, 5.0))?;
    let lower = drafted(m, lower, &[lower_side], 3.0, neutral, Direction::NEG_Z)?;
    Ok(boolean::union(m, lower, upper)?)
}
