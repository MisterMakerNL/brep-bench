//! A spindle torus about the z axis, middle circle radius 3, tube radius 5, counted once: the part
//! of the disc (x − 3)² + z² ≤ 25 with x ≥ 0 (its outer arc from (0, 0, −4) through (8, 0, 0) to
//! (0, 0, 4), closed along the z axis), turned a whole turn about the z axis. (solidcore's torus
//! primitive takes a ring torus only, its tube narrower than its middle circle.)
use crate::bench::arc;
use crate::shared::geometry::{axis, turned};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // angles in the xz plane from +x towards +z, about −y; the arc's ends at ±atan2(4, −3)
    let end = 4f64.atan2(-3.0);
    turned(vec![arc([3.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 5.0, -end, end), axis(4.0, -4.0)])
}
