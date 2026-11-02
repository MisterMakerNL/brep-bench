//! A sheet, the whole cylinder of radius 5 about the z axis from z = 0 to z = 10 (one face closed
//! on itself along a seam, bounded by its two circles), thickened by 1 away from the axis: the
//! tube between radii 5 and 6.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Cylinder, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // u round the axis over its whole period, v up it; its normal points away from the axis
    let cylinder = Surface::Cylinder(Cylinder::new(Frame::WORLD, 5.0));
    let sheet = make::face_on_domain(m, cylinder, 0.0, TAU, 0.0, 10.0)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
