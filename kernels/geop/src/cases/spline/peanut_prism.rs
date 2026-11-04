//! A prism 10 tall whose section is the region inside a closed, uniform, periodic cubic B-spline
//! of twelve control points.
//!
//! geop's sketch splines are clamped: each runs from its first control point to its last (a
//! drawn one on uniform clamped knots, one taken from elsewhere on clamped knots of its own). A
//! periodic spline, with no start or end, it cannot draw, and the clamped spline equal to it
//! would be control points computed here, not the curve the case gives.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no periodic spline: its sketch splines are clamped, running from their first \
         control point to their last, so a closed uniform periodic cubic B-spline cannot be drawn"
        .into())
}
