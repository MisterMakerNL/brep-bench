//! A round mounting flange on the z axis, its base on the plane z = 0: a plate, the disc of radius
//! 32 from z = 0 to z = 6; a hub, a cylinder of radius 15 from z = 6 to z = 16; a bore of radius 8
//! right through both; and six bolt holes of radius 3 straight through the plate, their axes on
//! the circle of radius 24, one every 60° from (24, 0). The drills run past the faces they cut.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::{Axis, Direction, Transform};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cylinder::new(32.0, 6.0).build(m)?;
    let hub = Cylinder::new(15.0, 10.0).with_frame(frame(p(0.0, 0.0, 6.0), Direction::Z)).build(m)?;
    let mut flange = boolean::union(m, plate, hub)?;
    let bore = Cylinder::new(8.0, 18.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    flange = boolean::difference(m, flange, bore)?;
    let first = Cylinder::new(3.0, 8.0).with_frame(frame(p(24.0, 0.0, -1.0), Direction::Z)).build(m)?;
    for k in 0..6 {
        let hole = first.moved(&Transform::rotation(Axis::Z, (60.0 * k as f64).to_radians()));
        flange = boolean::difference(m, flange, hole)?;
    }
    Ok(flange)
}
