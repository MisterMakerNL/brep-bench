//! A pipe of radius 3 along two straight runs, (−30, 0, 0) to the origin and on along +y to
//! (0, 30, 0), the disc carried round the sharp corner by turning it about the vertical line
//! through the corner (filling the wedge outside the corner with a quarter ball). The disc at
//! the start, piped with rounded transitions.
use crate::bench::p;
use crate::shared::sweep::{disc, polyline};
use kentron::math::Direction;
use kentron::sweep::{self, Corner, PipeOptions};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let path = polyline(m, &[p(-30.0, 0.0, 0.0), p(0.0, 0.0, 0.0), p(0.0, 30.0, 0.0)], false)?;
    let section = disc(m, p(-30.0, 0.0, 0.0), Direction::X, 3.0)?;
    sweep::pipe_cornered(m, section, path, &PipeOptions::default(), Corner::Rounded)
}
