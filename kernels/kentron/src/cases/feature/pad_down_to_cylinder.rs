//! A cylinder of radius 10 lying along the x axis, from x = −20 to x = 20, with a square pad on
//! top: the square |x| ≤ 3, |y| ≤ 3 in the plane z = 15, extruded down (along −z) until it meets
//! the cylinder's side, and joined to the cylinder. The pad is kentron's form feature, a boss
//! swept until the cylinder's side.
use crate::bench::{face_through, frame, p};
use crate::Refusal;
use kentron::feature::{self, FeatureExtent, FeatureMode, FeatureSweep};
use kentron::math::{Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::sketch::{self, Outcome, Step};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    use Step::{Start, To};
    let cylinder = Cylinder::new(10.0, 40.0).with_frame(frame(p(-20.0, 0.0, 0.0), Direction::X)).build(m)?;
    let sketch_plane = Frame::new(p(0.0, 0.0, 15.0), Direction::Z, Direction::X);
    let square = [Start(-3.0, -3.0), To(3.0, -3.0), To(3.0, 3.0), To(-3.0, 3.0)];
    let pad = sketch::profile(m, sketch_plane, &square, Outcome::Face)?;
    let side = face_through(m, cylinder, p(0.0, 0.0, 10.0))?;
    Ok(feature::feature(
        m,
        cylinder,
        pad,
        FeatureSweep::Prism(Direction::NEG_Z),
        FeatureMode::Add,
        FeatureExtent::Until(side),
    )?)
}
