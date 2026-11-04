//! A disc of radius 20 on the z axis from z = -5 to z = 0, with six holes of radius 2 through it
//! along z, their axes on the circle of radius 12: the first at (12, 0), the others it turned by
//! 60°, 120°, ... 300° about the z axis, each its own cut, each drill running past both faces.
use crate::bench::{cut_all, rotated};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let disc = make_cylinder([0.0, 0.0, -5.0], 20.0, 5.0)?;
    let drills = (0..6)
        .map(|i| {
            let first = make_cylinder([12.0, 0.0, -6.0], 2.0, 7.0)?;
            rotated(first, [0.0; 3], [0.0, 0.0, 1.0], 60.0 * f64::from(i))
        })
        .collect::<Result<Vec<_>, Refusal>>()?;
    cut_all(disc, drills)
}
