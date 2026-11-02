//! A picture frame: the rebated moulding (s outward, z) = (0, 2), (3, 2), (3, 0), (8, 0), (8, 6),
//! (0, 6) swept round the closed rectangle with corners (±30, ±20, 0), mitred at each corner in
//! the vertical plane along its diagonal. The section on the side y = −20, where (s, z) is at
//! (x, −20 − s, z), piped round the rectangle from there with mitred transitions.
use crate::bench::p;
use crate::shared::sweep::polyline;
use kentron::make;
use kentron::math::Point;
use kentron::sweep::{self, Corner, PipeOptions};
use kentron::topology::{Model, Shape};

const SECTION: [(f64, f64); 6] = [(0.0, 2.0), (3.0, 2.0), (3.0, 0.0), (8.0, 0.0), (8.0, 6.0), (0.0, 6.0)];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (w, h) = (30.0, 20.0);
    // Round the rectangle anticlockwise from the middle of its side y = −20, where the section
    // stands.
    let path = polyline(m, &[p(0.0, -h, 0.0), p(w, -h, 0.0), p(w, h, 0.0), p(-w, h, 0.0), p(-w, -h, 0.0)], true)?;
    let corners: Vec<Point> = SECTION.iter().map(|&(s, z)| p(0.0, -h - s, z)).collect();
    let rim = polyline(m, &corners, true)?;
    let section = make::planar_face(m, rim)?;
    sweep::pipe_cornered(m, section, path, &PipeOptions::default(), Corner::Mitred)
}
