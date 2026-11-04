//! A pipe of radius 8 along the twisted cubic Bézier (0, 0, 0), (12, 16, 0), (26, 8, 0),
//! (24, 0, 16): the disc of radius 8 square to the curve at its start (along (3, 4, 0)), swept
//! along the curve turning with it, its ends capped.
use crate::bench::circle;
use crate::shared::spline::{cubic, swept};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let path = cubic([[0.0, 0.0, 0.0], [12.0, 16.0, 0.0], [26.0, 8.0, 0.0], [24.0, 0.0, 16.0]]);
    let section = circle([0.0; 3], [0.6, 0.8, 0.0], [0.0, 0.0, 1.0], 8.0);
    swept(&[section], path)
}
