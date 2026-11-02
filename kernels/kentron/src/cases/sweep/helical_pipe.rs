//! A pipe of radius 1 along the helix about the z axis of radius 10 rising 5 a turn, from
//! (10, 0, 0), right-handed, for three whole turns; the section square to the path, the ends
//! flat: the disc square to the helix where it starts, piped along it in its Frenet frame.
use crate::bench::p;
use crate::shared::sweep::disc;
use kentron::helix::{helix, HelixOptions, HelixPart};
use kentron::math::{Direction, Frame};
use kentron::sweep::{self, PipeOptions};
use kentron::topology::{Model, Shape};

/// The helix, its B-spline turns within 1e-9 of the exact one.
fn path(m: &mut Model) -> kentron::Result<Shape> {
    let frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::X);
    let part = HelixPart { height: 15.0, pitch: 5.0, end_diameter: 20.0 };
    let options = HelixOptions { tolerance: 1e-9, ..Default::default() };
    Ok(helix(m, &frame, 20.0, &[part], &options)?.wire)
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let spine = path(m)?;
    // The helix leaves (10, 0, 0) along (0, 2π·10, 5).
    let section = disc(m, p(10.0, 0.0, 0.0), Direction::new(0.0, 20.0 * std::f64::consts::PI, 5.0), 1.0)?;
    let options = PipeOptions { frenet: true, ..Default::default() };
    Ok(sweep::pipe_with(m, section, spine, &options)?.shape)
}
