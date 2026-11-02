//! A pulley turned about the z axis, symmetric about z = 0: a hub, the cylinder of radius 12 from
//! z = −14 to z = 14; a rim, the disc of radius 28 from z = −9 to z = 9; a bore of radius 6 right
//! through (the drill from z = −15 to z = 15, past both ends); and a V-shaped belt groove round the
//! rim, the triangle with corners (x, z) = (22, 0), (29, −5) and (29, 5) in the xz plane turned a
//! full turn about the z axis and taken away. The triangle reaches 1 past the rim's face.
use crate::bench::{frame, p};
use crate::shared::parts::xz_plane;
use kentron::boolean;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Start, To};
    let hub = Cylinder::new(12.0, 28.0).with_frame(frame(p(0.0, 0.0, -14.0), Direction::Z)).build(m)?;
    let rim = Cylinder::new(28.0, 18.0).with_frame(frame(p(0.0, 0.0, -9.0), Direction::Z)).build(m)?;
    let mut pulley = boolean::union(m, hub, rim)?;
    let bore = Cylinder::new(6.0, 30.0).with_frame(frame(p(0.0, 0.0, -15.0), Direction::Z)).build(m)?;
    pulley = boolean::difference(m, pulley, bore)?;
    let triangle = sketch::profile(m, xz_plane(0.0), &[Start(22.0, 0.0), To(29.0, -5.0), To(29.0, 5.0)], Outcome::Face)?;
    let groove = sweep::revolve(m, triangle, Axis::Z, TAU)?.shape;
    boolean::difference(m, pulley, groove)
}
