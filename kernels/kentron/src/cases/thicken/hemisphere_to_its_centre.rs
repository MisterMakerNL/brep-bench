//! A sheet, the upper half of the sphere of radius 5 about the origin, thickened by 5 toward the
//! sphere's centre, its whole radius: the half ball of radius 5 where z ≥ 0.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Sphere, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // latitude v from 0 (the equator) to π/2 (the pole); the normal points away from the centre,
    // so toward it is a negative distance
    let sphere = Surface::Sphere(Sphere::new(Frame::WORLD, 5.0));
    let sheet = make::face_on_domain(m, sphere, 0.0, TAU, 0.0, FRAC_PI_2)?;
    thickened(m, sheet, -5.0, Join::Arc)
}
