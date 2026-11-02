//! A sheet, the half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to
//! z = 10, thickened by 1 away from the axis: the half of the tube between radii 5 and 6.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Cylinder, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the angle u from 0 to π is the half where y ≥ 0; the normal points away from the axis
    let cylinder = Surface::Cylinder(Cylinder::new(Frame::WORLD, 5.0));
    let sheet = make::face_on_domain(m, cylinder, 0.0, PI, 0.0, 10.0)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
