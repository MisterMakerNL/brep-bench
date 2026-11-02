//! A cube of side 10 between the origin and (10, 10, 10), united with a cylinder of radius 5 and
//! height 10 standing on the xy plane with its axis at x = 10, y = 5: half of it lies inside the
//! cube, and its side touches the cube's faces y = 0 and y = 10 along the edges at x = 10.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let boss = Cylinder::new(5.0, 10.0).with_frame(frame(p(10.0, 5.0, 0.0), Direction::Z)).build(m)?;
    boolean::union(m, cube, boss)
}
