//! A pipe of radius 2 along three edges of the cube of side 30, (0, 0, 0) to (30, 0, 0) to
//! (30, 30, 0) to (30, 30, 30), mitred at both corners in the planes bisecting them; the ends
//! flat and square to the path. The disc at the start, piped with mitred transitions.
use crate::bench::p;
use crate::shared::sweep::{disc, polyline};
use kentron::math::Direction;
use kentron::sweep::{self, Corner, PipeOptions};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let path = polyline(m, &[p(0.0, 0.0, 0.0), p(30.0, 0.0, 0.0), p(30.0, 30.0, 0.0), p(30.0, 30.0, 30.0)], false)?;
    let section = disc(m, p(0.0, 0.0, 0.0), Direction::X, 2.0)?;
    sweep::pipe_cornered(m, section, path, &PipeOptions::default(), Corner::Mitred)
}
