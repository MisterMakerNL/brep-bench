//! A closed sheet, the sphere of radius 5 about the origin as a surface alone, thickened by 1
//! outward: the hollow ball between the spheres of radius 5 and 6.
use crate::Refusal;
use crate::shared::thicken::thickened;
use kentron::geometry::surface::{Sphere, Surface};
use kentron::make;
use kentron::math::Frame;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // the whole sphere, one face over all its parameters; its normal points away from the centre
    let sphere = Surface::Sphere(Sphere::new(Frame::WORLD, 5.0));
    let sheet = make::face_on_surface(m, sphere)?;
    thickened(m, sheet, 1.0, Join::Arc)
}
