//! A drum, the cylinder of radius 10 on the z axis from z = 0 to z = 10, with a groove round its
//! side from z = 4.2 to z = 5.8 cut to radius 7, grown by 1: every point within 1 of it. The
//! groove, 1.6 wide, fills, leaving a shallow V-shaped notch: two pieces of tori meeting in the
//! circle of radius 10.6 at z = 5. The groove is cut as a ring reaching past the drum's side.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let drum = Cylinder::new(10.0, 10.0).build(m)?;
    let at = frame(p(0.0, 0.0, 4.2), Direction::Z);
    let disc = Cylinder::new(11.0, 1.6).with_frame(at).build(m)?;
    let core = Cylinder::new(7.0, 1.6).with_frame(at).build(m)?;
    let ring = boolean::difference(m, disc, core)?;
    let grooved = boolean::difference(m, drum, ring)?;
    offset::shape(m, grooved, 1.0, Join::Arc)
}
