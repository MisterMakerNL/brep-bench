//! The drum of `geometry/saddle-drum` (radius 8, floor z = 0, top the bilinear B-spline saddle
//! z = 10 + xy/8) bored through along its axis: a cylinder of radius 3 running past the floor and
//! the saddle, cut from it.
use crate::bench::cut;
use crate::shared::geometry::saddle_drum;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let bore = make_cylinder([0.0, 0.0, -1.0], 3.0, 20.0)?;
    cut(saddle_drum()?, bore)
}
