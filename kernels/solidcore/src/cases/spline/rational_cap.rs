//! A ball's cap: the region of the xz plane bounded by the z axis from (0, 0, −5) to (0, 0, 10),
//! the segment from (0, 0, −5) to (5√3, 0, −5) and the rational quadratic Bézier from
//! (5√3, 0, −5) about (10√3, 0, 10) to (0, 0, 10), weighted 1, ½ and 1, turned a whole turn about
//! the z axis.
use crate::bench::line;
use crate::shared::geometry::{axis, conic, turned};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let s = 3f64.sqrt();
    turned(vec![
        line([0.0, 0.0, -5.0], [5.0 * s, 0.0, -5.0]),
        conic([5.0 * s, 0.0, -5.0], [10.0 * s, 0.0, 10.0], 0.5, [0.0, 0.0, 10.0]),
        axis(10.0, -5.0),
    ])
}
