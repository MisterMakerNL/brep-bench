//! A sheet, the cap of the sphere of radius 10 about the origin where z ≥ 6, thickened by 2 away
//! from the sphere's centre: the solid between the spheres of radius 10 and 12 inside the cone
//! about the z axis, its apex at the origin, that the cap's rim sweeps.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Sphere, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // latitude v from where z = 6 to the pole; the normal points away from the centre
    let sphere = Surface::Sphere(Sphere::new(Frame::WORLD, 10.0));
    let sheet = make::face_on_domain(m, sphere, 0.0, TAU, (6.0f64 / 10.0).asin(), FRAC_PI_2)?;
    thickened(m, sheet, 2.0, Join::Arc)
}
