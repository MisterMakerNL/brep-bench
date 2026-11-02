//! A closed hollow dome with a post inside it.
//!
//! The dome: a plate, the disc of radius 20 about the z axis from z = −2 to z = 0, and on it a
//! hemispherical shell, everything above z = 0 between 18 and 20 from the origin, enclosing a
//! cavity, the half ball of radius 18 above the plate.
//!
//! The post: the disc of radius 3 about the z axis on the cavity's floor (z = 0), extruded up
//! until it meets the inside of the dome, the sphere of radius 18, and joined to the dome. It is
//! kentron's form feature, a boss swept along +z until the dome's inside face.
use crate::bench::{face_through, frame, p};
use crate::Refusal;
use kentron::boolean;
use kentron::feature::{self, FeatureExtent, FeatureMode, FeatureSweep};
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cylinder::new(20.0, 2.0).with_frame(frame(p(0.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let outer = Sphere::new(20.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let inner = Sphere::new(18.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let shell = boolean::difference(m, outer, inner)?;
    let dome = boolean::union(m, plate, shell)?;

    let circle = make::edge_from_curve(m, Curve::Circle(Circle::new(frame(p(0.0, 0.0, 0.0), Direction::Z), 3.0)))?;
    let disc = make::wire(m, &[circle])?;
    let inside = face_through(m, dome, p(0.0, 0.0, 18.0))?;
    Ok(feature::feature(
        m,
        dome,
        disc,
        FeatureSweep::Prism(Direction::Z),
        FeatureMode::Add,
        FeatureExtent::Until(inside),
    )?)
}
