//! The solid inside the upper sheet of the hyperboloid of two sheets z²/9 − (x² + y²)/16 = 1,
//! below z = 6: the hyperbola z²/9 − x²/16 = 1 of the xz plane from its vertex (0, 0, 3) out to
//! the rim (4√3, 0, 6), closed by the top's radius and the axis, turned a whole turn about z.
use crate::bench::p;
use crate::shared::geometry::{arc, flat, segment, turned};
use kentron::geometry::curve::{Curve, Hyperbola};
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

const A: f64 = 4.0; // z²/C² − (x² + y²)/A² = 1
const C: f64 = 3.0;
const H: f64 = 6.0; // the top, a flat disc at z = H

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // (C cosh t, A sinh t) along the frame's x and y: +z, and +x when its normal is +y
    let frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Y, Direction::Z);
    let branch = Curve::Hyperbola(Hyperbola::new(frame, C, A));
    let end = (H / C).acosh();
    let side = arc(m, branch, 0.0, end)?;
    let rim = A * ((H / C).powi(2) - 1.0).sqrt();
    let top = segment(m, p(rim, 0.0, H), p(0.0, 0.0, H))?;
    let axis = segment(m, p(0.0, 0.0, H), p(0.0, 0.0, C))?;
    let section = flat(m, &[side, top, axis])?;
    turned(m, section)
}
