//! A pipe of circular section, radius 8, swept along the cubic Bézier curve with control points
//! (0, 0, 0), (12, 16, 0), (26, 8, 0) and (24, 0, 16), which does not lie in a plane: at every
//! point the section is the disc of radius 8 centred on the curve, square to it, and the ends are
//! flat discs. A disc's turning about the curve does not change the pipe.
use crate::shared::spline::{bezier, round_pipe};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let path = bezier(&[(0.0, 0.0, 0.0), (12.0, 16.0, 0.0), (26.0, 8.0, 0.0), (24.0, 0.0, 16.0)]);
    round_pipe(m, path, 8.0)
}
