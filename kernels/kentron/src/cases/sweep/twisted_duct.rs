//! A solid duct along the y axis from y = 0 to y = 40, from the square A (10, 0, −10),
//! B (−10, 0, −10), C (−10, 0, 10), D (10, 0, 10) to the diamond A′ (0, 40, −12), B′ (−12, 40, 0),
//! C′ (0, 40, 12), D′ (12, 40, 0), each corner running straight to its primed one, each side a
//! twisted ruled face: a ruled loft of the two polygons, their edges paired as given.
use crate::bench::p;
use crate::shared::sweep::polyline;
use kentron::loft::{loft, LoftOptions, Matching};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let square = polyline(m, &[p(10.0, 0.0, -10.0), p(-10.0, 0.0, -10.0), p(-10.0, 0.0, 10.0), p(10.0, 0.0, 10.0)], true)?;
    let diamond = polyline(m, &[p(0.0, 40.0, -12.0), p(-12.0, 40.0, 0.0), p(0.0, 40.0, 12.0), p(12.0, 40.0, 0.0)], true)?;
    // Each wire starts at its A and runs the same way round, and is taken as it is: A meets A′.
    let options = LoftOptions { solid: true, ruled: true, matching: Matching::AsGiven, ..Default::default() };
    Ok(loft(m, &[square, diamond], &options)?.shape)
}
