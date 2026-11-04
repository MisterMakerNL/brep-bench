//! A C-shaped bracket, the polygon through (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24),
//! (40, 24), (40, 30), (0, 30) in the plane y = −10 extruded along +y to y = 10, with a hole of
//! radius 4 on the vertical axis through (28, 0) drilled down from the upper arm's top through
//! the first wall it meets, the upper arm, ending in the gap; the lower arm is not drilled.
//!
//! solidcore-ops has no hole that runs to the next face it comes out of, so the drill is a
//! cylinder from inside the gap, z = 20, up past the arm's top, through the upper arm only.
use crate::bench::{cut, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let section = polygon(&[
        [0.0, -10.0, 0.0],
        [40.0, -10.0, 0.0],
        [40.0, -10.0, 6.0],
        [6.0, -10.0, 6.0],
        [6.0, -10.0, 24.0],
        [40.0, -10.0, 24.0],
        [40.0, -10.0, 30.0],
        [0.0, -10.0, 30.0],
    ]);
    let bracket = extrude(&section, [0.0, 1.0, 0.0], 20.0, 0.0, DraftCorner::default(), true)?;
    let drill = make_cylinder([28.0, 0.0, 20.0], 4.0, 12.0)?;
    cut(bracket, drill)
}
