//! The block from the origin to (40, 30, 15) with a pocket cut into its top: the box from
//! (8, 8, 5) to (32, 22, 15) with its four vertical edges rounded to radius 4. The pocket's eight
//! walls (four planes, four concave quarter cylinders) are drafted 5° with the pull direction +z
//! about the neutral plane z = 5, its floor, each leaning away from the pocket's middle as it
//! rises; each round becomes a quarter cone about the same axis, tangent to the planes beside it.
use crate::bench::p;
use crate::shared::draft::{box_sides, drafted, rounded_box};
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(40.0, 30.0, 15.0)).build(m)?;
    let (lo, hi) = (p(8.0, 8.0, 5.0), p(32.0, 22.0, 15.0));
    // The pocket's box runs past the top, so that the cut leaves no sliver of the top face.
    let pocket = rounded_box(m, lo, p(32.0, 22.0, 16.0), 4.0)?;
    let part = boolean::difference(m, block, pocket)?;
    // A wall facing into the pocket leans toward the block's material for a positive angle.
    let walls = box_sides(m, part, lo, hi, 4.0)?;
    drafted(m, part, &walls, 5.0, lo, Direction::Z)
}
