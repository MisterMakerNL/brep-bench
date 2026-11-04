//! A pipe of radius 8 along the cubic Bézier curve with control points (0, 0, 0), (12, 16, 0),
//! (26, 8, 0) and (24, 0, 16), which does not lie in a plane.
//!
//! geop draws a curve through space only in a 3-D sketch, whose splines interpolate the points
//! they are drawn through (a natural cubic spline, its ends' directions at most given); a spline
//! of control points, which a Bézier curve is, it draws only in a plane. So the path cannot be
//! drawn.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no space curve given by control points: its 3-D sketch's splines interpolate \
         the points they pass through, and a spline of control points (a Bézier curve) is drawn \
         only in a plane sketch, so this non-planar cubic Bézier path cannot be drawn"
        .into())
}
