//! A lemon: the segment of the disc of radius 5 about (−3, 0, 0) in the xz plane with x ≥ 0 (its
//! arc from (0, 0, −4) through (2, 0, 0) to (0, 0, 4), closed by the chord along the z axis),
//! turned a whole turn about the z axis.
use crate::bench::arc;
use crate::shared::geometry::{axis, turned};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // angles in the xz plane from +x towards +z, about −y; the chord's ends at ±atan2(4, 3)
    let end = 4f64.atan2(3.0);
    turned(vec![arc([-3.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 5.0, -end, end), axis(4.0, -4.0)])
}
