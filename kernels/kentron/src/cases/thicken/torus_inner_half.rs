//! A sheet, the inner half of the torus about the z axis whose tube has radius 2 about a middle
//! circle of radius 6 (its points no farther than 6 from the axis), thickened by 1 away from the
//! middle circle, toward the axis.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Surface, Torus};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // v round the tube from π/2 (its top) through π (nearest the axis) to 3π/2 (its bottom); the
    // normal points away from the middle circle
    let torus = Surface::Torus(Torus::new(Frame::WORLD, 6.0, 2.0));
    let sheet = make::face_on_domain(m, torus, 0.0, TAU, FRAC_PI_2, PI + FRAC_PI_2)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
