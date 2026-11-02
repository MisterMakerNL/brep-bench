//! A hollow square tube tapering along the y axis from y = 0 to y = 40: outside 24 × 24 with a
//! 20 × 20 hole at y = 0, 12 × 12 with an 8 × 8 hole at y = 40, every corner running straight
//! from end to end. The outside's ruled loft, less the hole's, which runs exactly end to end.
use crate::bench::p;
use crate::shared::sweep::polyline;
use kentron::boolean;
use kentron::loft::{loft, LoftOptions};
use kentron::topology::{Model, Shape};

/// The square of side `side` centred on the y axis at `y`, its sides along x and z.
fn square(m: &mut Model, side: f64, y: f64) -> kentron::Result<Shape> {
    let h = side / 2.0;
    polyline(m, &[p(-h, y, -h), p(h, y, -h), p(h, y, h), p(-h, y, h)], true)
}

/// The solid square frustum from side `start` at y = 0 to `end` at y = 40, ruled.
fn frustum(m: &mut Model, start: f64, end: f64) -> kentron::Result<Shape> {
    let sections = [square(m, start, 0.0)?, square(m, end, 40.0)?];
    Ok(loft(m, &sections, &LoftOptions { solid: true, ruled: true, ..Default::default() })?.shape)
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let outside = frustum(m, 24.0, 12.0)?;
    let hole = frustum(m, 20.0, 8.0)?;
    boolean::difference(m, outside, hole)
}
