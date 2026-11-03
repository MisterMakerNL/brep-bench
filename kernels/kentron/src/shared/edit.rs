//! What the edit implementations share: a block drilled right through along z, and faces moved.
//!
//! kentron moves a face by offsetting it (`offset::shape_with_faces`, OCCT's SetOffsetOnFace):
//! the solid as a whole is offset by nothing and each face given by its own distance, along its
//! outward normal, the faces beside it extended or trimmed to meet it where their surfaces
//! intersect. A plane moved along its normal is the plane translated, and a cylinder offset is
//! the cylinder about the same axis with its radius changed.
use crate::bench::{frame, p};
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};
use kentron::{boolean, Result};

/// The box from the origin to `corner`, drilled right through along z by a hole of `radius`
/// about the vertical line through (`x`, `y`), the drill running 1 past the floor and the top.
pub fn drilled_block(m: &mut Model, corner: [f64; 3], x: f64, y: f64, radius: f64)
    -> Result<Shape> {
    let [a, b, c] = corner;
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(a, b, c)).build(m)?;
    let axis = frame(p(x, y, -1.0), Direction::Z);
    let drill = Cylinder::new(radius, c + 2.0).with_frame(axis).build(m)?;
    boolean::difference(m, block, drill)
}

/// `solid` with each (face, distance) of `moves` moved that far along its outward normal, and
/// the faces beside it extended or trimmed to meet it.
pub fn moved(m: &mut Model, solid: Shape, moves: &[(Shape, f64)]) -> Result<Shape> {
    offset::shape_with_faces(m, solid, 0.0, moves, Join::Intersection)
}
