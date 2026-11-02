//! A block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8, with a hole x²/36 + y²/9 < 1 bored straight
//! through it along z. The drill is the ellipse at z = −1 extruded to z = 9, past both faces.
use crate::bench::{p, v};
use crate::shared::geometry::flat;
use kentron::boolean;
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-12.0, -8.0, 0.0), p(12.0, 8.0, 8.0)).build(m)?;
    let frame = Frame::new(p(0.0, 0.0, -1.0), Direction::Z, Direction::X);
    let rim = make::edge_from_curve(m, Curve::Ellipse(Ellipse::new(frame, 6.0, 3.0)))?;
    let disc = flat(m, &[rim])?;
    let drill = sweep::extrude(m, disc, v(0.0, 0.0, 10.0))?.shape;
    boolean::difference(m, block, drill)
}
