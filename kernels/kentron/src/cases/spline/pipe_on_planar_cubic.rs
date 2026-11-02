//! A pipe of circular section, radius 20, swept along the planar cubic Bézier curve with control
//! points (0, 0, 0), (12, −16, 0), (32, −16, 0) and (44, 0, 0): at every point the section is the
//! disc of radius 20 centred on the curve, square to it, and the ends are flat discs. The radius
//! is close to the curve's least radius of curvature, 24, but the wall never folds.
use crate::shared::spline::{bezier, round_pipe};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let path = bezier(&[(0.0, 0.0, 0.0), (12.0, -16.0, 0.0), (32.0, -16.0, 0.0), (44.0, 0.0, 0.0)]);
    round_pipe(m, path, 20.0)
}
