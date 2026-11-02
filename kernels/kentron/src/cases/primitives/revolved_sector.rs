//! A quarter of a washer: the rectangle with x from 6 to 10 and z from 0 to 4 in the xz plane,
//! revolved 90° about the z axis, right-handed (from +x towards +y).
use crate::shared::primitives::rectangle_xz;
use kentron::math::Axis;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let section = rectangle_xz(m, (6.0, 0.0), (10.0, 4.0))?;
    Ok(sweep::revolve(m, section, Axis::Z, 90f64.to_radians())?.shape)
}
