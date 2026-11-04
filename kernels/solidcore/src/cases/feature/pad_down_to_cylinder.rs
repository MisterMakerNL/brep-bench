//! A cylinder of radius 10 lying along the x axis from x = −20 to x = 20, and a square pad on
//! top: the square |x| ≤ 3, |y| ≤ 3 in the plane z = 15 extruded down until it meets the
//! cylinder's side, joined to the cylinder.
//!
//! solidcore-ops has no extrusion up to a face, so the square is extruded down past the side,
//! into the cylinder (to z = 5, where the cylinder is wider than the square), and united with
//! it: the part inside the cylinder is the cylinder's already.
use crate::bench::{fuse, polygon, rotated};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    // a cylinder on z from z = −20 to z = 20, turned onto x about y
    let cylinder = make_cylinder([0.0, 0.0, -20.0], 10.0, 40.0)?;
    let cylinder = rotated(cylinder, [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 90.0)?;
    let square =
        polygon(&[[-3.0, -3.0, 15.0], [3.0, -3.0, 15.0], [3.0, 3.0, 15.0], [-3.0, 3.0, 15.0]]);
    let pad = extrude(&square, [0.0, 0.0, -1.0], 10.0, 0.0, DraftCorner::default(), true)?;
    fuse(cylinder, pad)
}
