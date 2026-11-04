//! A pipe of radius 20 along the planar cubic Bézier (0, 0, 0), (12, −16, 0), (32, −16, 0),
//! (44, 0, 0): the disc of radius 20 square to the curve at its start (along (3, −4, 0)), swept
//! along the curve turning with it, its ends capped.
use crate::bench::circle;
use crate::shared::spline::{cubic, swept};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let path = cubic([[0.0, 0.0, 0.0], [12.0, -16.0, 0.0], [32.0, -16.0, 0.0], [44.0, 0.0, 0.0]]);
    let section = circle([0.0; 3], [0.6, -0.8, 0.0], [0.0, 0.0, 1.0], 20.0);
    swept(&[section], path)
}
