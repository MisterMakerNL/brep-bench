//! The solid bounded by two flat discs of radius 5 about the z axis, at z = −5 and z = 5, and the
//! surface ruled between their rims turned a quarter turn against each other: a loft of degree 1
//! (a ruled surface) from the lower circle, starting at angle 0, to the upper, starting at 90°,
//! so the point at angle θ below is joined straight to the one at θ + 90° above.
use crate::bench::circle;
use crate::Refusal;
use solidcore_ops::sweep::loft;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let lower = circle([0.0, 0.0, -5.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 5.0);
    let upper = circle([0.0, 0.0, 5.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], 5.0);
    Ok(loft(&[vec![lower], vec![upper]], 1, true)?)
}
