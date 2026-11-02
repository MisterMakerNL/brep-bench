//! A sheet, the side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at
//! z = 4, thickened by 1 along its normal away from the axis, which leans up. Its rims are the
//! frustums its boundary circles sweep.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Cone, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // radius 6 at the cone's origin, narrowing as it rises: a negative half angle; v runs along
    // the slant, 5 from rim to rim, and the normal points away from the axis
    let cone = Surface::Cone(Cone::new(Frame::WORLD, -(3.0f64).atan2(4.0), 6.0));
    let sheet = make::face_on_domain(m, cone, 0.0, TAU, 0.0, 5.0)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
