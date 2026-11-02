//! What the primitives implementations share: a rectangular section in the xz plane.
use crate::bench::p;
use kentron::math::{Direction, Frame};
use kentron::sketch::{self, Outcome, Step};
use kentron::topology::{Model, Shape};

/// The face of the rectangle in the xz plane between the corners `(x, z)` `low` and `high`,
/// drawn on the plane with u along x and v along z.
pub fn rectangle_xz(m: &mut Model, low: (f64, f64), high: (f64, f64)) -> kentron::Result<Shape> {
    use Step::{Start, To};
    let xz = Frame::new(p(0.0, 0.0, 0.0), Direction::NEG_Y, Direction::X);
    let outline = [Start(low.0, low.1), To(high.0, low.1), To(high.0, high.1), To(low.0, high.1)];
    sketch::profile(m, xz, &outline, Outcome::Face)
}
