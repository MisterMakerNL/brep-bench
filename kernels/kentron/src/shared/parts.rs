//! What the parts implementations share: the plane their outlines in the xz plane are drawn on.
use crate::bench::p;
use kentron::math::{Direction, Frame};

/// The plane parallel to the xz plane at `y`, its u along x and its v along z (its normal −y).
pub fn xz_plane(y: f64) -> Frame {
    Frame::new(p(0.0, y, 0.0), Direction::NEG_Y, Direction::X)
}
