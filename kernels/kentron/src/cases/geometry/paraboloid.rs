//! A solid paraboloid of revolution about the z axis, vertex at the origin, filled up to z = 8:
//! the parabola x² = 8z of the xz plane from its vertex to (8, 0, 8), closed by the top's radius
//! and the axis, turned a whole turn about the z axis.
use crate::bench::p;
use crate::shared::geometry::{arc, flat, segment, turned};
use kentron::geometry::curve::{Curve, Parabola};
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

const FOCAL: f64 = 2.0; // x² + y² = 4·FOCAL·z
const H: f64 = 8.0; // the top, a flat disc at z = H

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // (t²/(4·FOCAL), t) along the frame's x and y: +z, and +x when its normal is +y
    let frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Y, Direction::Z);
    let parabola = Curve::Parabola(Parabola::new(frame, FOCAL));
    let rim = (4.0 * FOCAL * H).sqrt();
    let side = arc(m, parabola, 0.0, rim)?;
    let top = segment(m, p(rim, 0.0, H), p(0.0, 0.0, H))?;
    let axis = segment(m, p(0.0, 0.0, H), p(0.0, 0.0, 0.0))?;
    let section = flat(m, &[side, top, axis])?;
    turned(m, section)
}
