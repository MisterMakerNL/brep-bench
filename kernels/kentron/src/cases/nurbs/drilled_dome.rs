//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with a round hole of radius 3
//! drilled straight down through it about the line x = 12, y = 8: the drill is a cylinder that
//! runs past the floor and past the top.
use crate::bench::{frame, p};
use crate::shared::nurbs::bezier_dome;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = bezier_dome(m)?;
    let drill = Cylinder::new(3.0, 40.0).with_frame(frame(p(12.0, 8.0, -5.0), Direction::Z)).build(m)?;
    boolean::difference(m, block, drill)
}
