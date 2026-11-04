//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, drilled straight down by a
//! hole of radius 3 about the line x = 12, y = 8: a cylinder from below the floor to above the
//! dome, cut from the block.
use crate::bench::cut;
use crate::shared::nurbs::bezier_dome;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = bezier_dome()?;
    let drill = make_cylinder([12.0, 8.0, -1.0], 3.0, 30.0)?;
    cut(block, drill)
}
