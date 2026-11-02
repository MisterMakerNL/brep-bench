//! The box from the origin to (20, 12, 10), its side x = 20 alone drafted 10° with the pull
//! direction +z about the neutral plane z = 0, leaning toward −x as it rises: a prism along y
//! whose section is the trapezoid (0, 0), (20, 0), (20 − 10·tan 10°, 10), (0, 10) in xz.
use crate::bench::{face_through, p};
use crate::shared::draft::drafted;
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 12.0, 10.0)).build(m)?;
    let side = face_through(m, block, p(20.0, 6.0, 5.0))?;
    drafted(m, block, &[side], 10.0, p(0.0, 0.0, 0.0), Direction::Z)
}
