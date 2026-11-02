//! An open tray lying on its back, its opening facing −y, made in three steps:
//! 1. a block from (−36, 0, −24) to (36, 16, 24);
//! 2. the pocket, the box from (−33, 0, −21) to (33, 13, 21), taken out of it, leaving walls 3
//!    thick and a floor from y = 13 to y = 16, open through the face at y = 0;
//! 3. four standoffs joined to it on the floor: tubes along y, outer radius 5 and inner radius 2,
//!    from y = 5 to y = 13, their axes through (x, z) = (±26, ±14). Each hole is blind, stopping
//!    at the floor.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-36.0, 0.0, -24.0), p(36.0, 16.0, 24.0)).build(m)?;
    let pocket = Cuboid::from_corners(p(-33.0, 0.0, -21.0), p(33.0, 13.0, 21.0)).build(m)?;
    let mut tray = boolean::difference(m, block, pocket)?;
    for (x, z) in [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)] {
        let axis = frame(p(x, 5.0, z), Direction::Y);
        let outer = Cylinder::new(5.0, 8.0).with_frame(axis).build(m)?;
        let inner = Cylinder::new(2.0, 8.0).with_frame(axis).build(m)?;
        let standoff = boolean::difference(m, outer, inner)?;
        tray = boolean::union(m, tray, standoff)?;
    }
    Ok(tray)
}
