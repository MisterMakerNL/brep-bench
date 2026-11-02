//! A hub with a stop, and a rib turned round the hub until it meets the stop. Angles about the z
//! axis are measured from +x, turning towards +y.
//!
//! - The hub: a cylinder of radius 10 on the z axis, from z = 0 to z = 10.
//! - The stop: everything within 25 of the z axis between the half-planes at 120° and 135°, from
//!   z = 0 to z = 10, joined to the hub.
//! - The rib: the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 in the plane y = 0, turned about the z axis
//!   from 0° towards +y until it meets the stop's face at 120°, and joined to them.
//!
//! The rib is kentron's revolved form feature, a boss turned about the z axis until the stop's
//! face; the stop is a 15° sector of a cylinder.
use crate::bench::{d, face_through, p};
use crate::Refusal;
use kentron::boolean;
use kentron::feature::{self, FeatureExtent, FeatureMode, FeatureSweep};
use kentron::math::{Axis, Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::sketch::{self, Outcome, Step};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    use Step::{Start, To};
    let (from, to) = (120f64.to_radians(), 135f64.to_radians());
    let hub = Cylinder::new(10.0, 10.0).build(m)?;
    let stop_frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, d(from.cos(), from.sin(), 0.0));
    let stop = Cylinder::new(25.0, 10.0).with_frame(stop_frame).with_angle(to - from).build(m)?;
    let base = boolean::union(m, hub, stop)?;

    // The rectangle in the plane y = 0, u along x and v along z.
    let xz = Frame::new(p(0.0, 0.0, 0.0), Direction::NEG_Y, Direction::X);
    let rectangle = [Start(5.0, 0.0), To(20.0, 0.0), To(20.0, 4.0), To(5.0, 4.0)];
    let rib = sketch::profile(m, xz, &rectangle, Outcome::Face)?;
    let stop_face = face_through(m, base, p(17.5 * from.cos(), 17.5 * from.sin(), 5.0))?;
    Ok(feature::feature(m, base, rib, FeatureSweep::Revolution(Axis::Z), FeatureMode::Add, FeatureExtent::Until(stop_face))?)
}
