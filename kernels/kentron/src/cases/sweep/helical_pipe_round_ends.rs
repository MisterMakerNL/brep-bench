//! The helical pipe (radius 1, along the helix of radius 10 about the z axis rising 5 a turn,
//! from (10, 0, 0), right-handed, three turns), each end closed by a half ball of radius 1: the
//! pipe with a ball fused on at each end of the helix, half of each inside the pipe.
use crate::bench::p;
use crate::shared::sweep::disc;
use kentron::boolean;
use kentron::helix::{helix, HelixOptions, HelixPart};
use kentron::math::{Direction, Frame};
use kentron::primitives::Sphere;
use kentron::sweep::{self, PipeOptions};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::X);
    let part = HelixPart { height: 15.0, pitch: 5.0, end_diameter: 20.0 };
    let options = HelixOptions { tolerance: 1e-9, ..Default::default() };
    let spine = helix(m, &frame, 20.0, &[part], &options)?.wire;
    // The helix leaves (10, 0, 0) along (0, 2π·10, 5), and ends there three turns up.
    let section = disc(m, p(10.0, 0.0, 0.0), Direction::new(0.0, 20.0 * std::f64::consts::PI, 5.0), 1.0)?;
    let options = PipeOptions { frenet: true, ..Default::default() };
    let mut body = sweep::pipe_with(m, section, spine, &options)?.shape;
    for centre in [p(10.0, 0.0, 0.0), p(10.0, 0.0, 15.0)] {
        let ball = Sphere::new(1.0).with_frame(Frame::from_z(centre, Direction::Z)).build(m)?;
        body = boolean::union(m, body, ball)?;
    }
    Ok(body)
}
