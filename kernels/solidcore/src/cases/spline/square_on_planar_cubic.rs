//! A bar of square section 20 × 20 along the planar cubic Bézier (0, 0, 0), (16, 12, 0),
//! (18, 26, 0), (12, 34, 0): the square centred on the curve's start, square to it there (along
//! (4, 3, 0)), two sides horizontal, swept along the curve turning with it, its ends capped.
use crate::bench::polygon;
use crate::shared::spline::{cubic, swept};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let path = cubic([[0.0, 0.0, 0.0], [16.0, 12.0, 0.0], [18.0, 26.0, 0.0], [12.0, 34.0, 0.0]]);
    // 10 to the curve's left, (−3, 4, 0)/5, and 10 up
    let (lx, ly) = (-6.0, 8.0);
    let section = polygon(&[[lx, ly, -10.0], [lx, ly, 10.0], [-lx, -ly, 10.0], [-lx, -ly, -10.0]]);
    swept(&section, path)
}
