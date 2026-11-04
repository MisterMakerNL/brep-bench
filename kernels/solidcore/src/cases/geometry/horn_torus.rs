//! A solid horn torus about the z axis, middle circle and tube both of radius 5: the disc
//! (x − 5)² + z² ≤ 25 of the xz plane, which touches the axis at the origin, turned a whole turn
//! about the z axis. (solidcore's torus primitive takes a ring torus only, its tube narrower than
//! its middle circle.)
use crate::bench::circle;
use crate::shared::geometry::turned;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the circle starts and ends where it touches the axis, at the origin
    turned(vec![circle([5.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], 5.0)])
}
