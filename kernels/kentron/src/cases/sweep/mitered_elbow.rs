//! A square bar 10 × 10 swept along two straight runs, (0, 0, 0) to (40, 0, 0) and on, turned
//! 120° to the left, to (20, 20√3, 0), mitred at the corner in the vertical plane bisecting it;
//! the section square to the path, two sides horizontal, the ends flat. The square at the start,
//! piped round the corner with mitred transitions.
use crate::bench::p;
use crate::shared::sweep::polyline;
use kentron::make;
use kentron::sweep::{self, Corner, PipeOptions};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let turn = 120f64.to_radians(); // to the left, from +x towards +y
    let path = polyline(m, &[p(0.0, 0.0, 0.0), p(40.0, 0.0, 0.0), p(40.0 + 40.0 * turn.cos(), 40.0 * turn.sin(), 0.0)], false)?;
    // The square at the path's start, in the plane x = 0, its sides at y, z = ±5. (Turned at the
    // corner about the vertical, the least turn from one run to the next, it stays upright.)
    let rim = polyline(m, &[p(0.0, -5.0, -5.0), p(0.0, 5.0, -5.0), p(0.0, 5.0, 5.0), p(0.0, -5.0, 5.0)], true)?;
    let square = make::planar_face(m, rim)?;
    sweep::pipe_cornered(m, square, path, &PipeOptions::default(), Corner::Mitred)
}
