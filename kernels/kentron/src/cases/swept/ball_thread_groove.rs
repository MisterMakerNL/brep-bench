//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! cut round it by a ball of radius 1 whose centre follows the helix of radius 5 about the z axis
//! from (5, 0, −2), right-handed, rising 4 a turn, for six turns to (5, 0, 22): everything the
//! ball passes through, cut.
//!
//! kentron has no sweep of a solid (`sweep::pipe` refuses one: "a pipe of a Solid"), so the ball's
//! sweep is built as elementary geometry gives it: a point p of the ball, centre c, moving with it
//! has velocity v + ω × (p − c), which runs along the sphere exactly where v·(p − c) = 0, on the
//! great circle square to the path. So what the ball passes through is the tube that circle's
//! disc sweeps, kept square to the helix, with the ball where it starts and where it ends. The
//! helix turns no tighter than about 5.08, and its turns are 4 apart, so the tube neither folds
//! nor meets itself.
use crate::bench::p;
use crate::shared::sweep::disc;
use crate::shared::swept::cut_each;
use kentron::helix::{helix, HelixOptions, HelixPart};
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cylinder, Sphere};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    // The helix, its B-spline turns within 1e-9 of the exact one: from (5, 0, −2), 24 up.
    let frame = Frame::new(p(0.0, 0.0, -2.0), Direction::Z, Direction::X);
    let part = HelixPart { height: 24.0, pitch: 4.0, end_diameter: 10.0 };
    let path = helix(m, &frame, 10.0, &[part], &HelixOptions { tolerance: 1e-9, ..Default::default() })?.wire;
    // The great circle's disc where the helix starts, square to it: it leaves along (0, 2π·5, 4).
    let section = disc(m, p(5.0, 0.0, -2.0), Direction::new(0.0, 10.0 * PI, 4.0), 1.0)?;
    let tube = sweep::pipe(m, section, path)?.shape;
    let start = Sphere::new(1.0).with_frame(Frame::from_z(p(5.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let end = Sphere::new(1.0).with_frame(Frame::from_z(p(5.0, 0.0, 22.0), Direction::Z)).build(m)?;
    // The tube and each ball cut from the rod in turn (both balls are clear of it).
    cut_each(m, rod, &[tube, start, end])
}
