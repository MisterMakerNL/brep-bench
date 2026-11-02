//! A solid elliptic cylinder standing on the xy plane: the ellipse x²/25 + y²/9 ≤ 1 in the plane
//! z = 0, extruded straight up to z = 10.
use crate::bench::v;
use crate::shared::geometry::flat;
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::make;
use kentron::math::Frame;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // the world frame: the major axis (5) along +x, the minor (3) along +y
    let rim = make::edge_from_curve(m, Curve::Ellipse(Ellipse::new(Frame::WORLD, 5.0, 3.0)))?;
    let end = flat(m, &[rim])?;
    Ok(sweep::extrude(m, end, v(0.0, 0.0, 10.0))?.shape)
}
